use std::str::FromStr;
use cyfs_lib::*;
use cyfs_base::*;
use log::*;
use async_std::sync::Arc;
use serde_json::{json, Value};
use crate::*;




/// -----------------------------------------------------
/// controller base method
/// 
/// 

/// sort_list_by_date
/// 根据日期排序
pub fn sort_list_by_date(a: &Value, b: &Value) -> std::cmp::Ordering {
    // info!("sort_by {:?},  {:?}", b["date"].as_i64(), a["date"].as_i64());
    a["date"].as_i64().unwrap().cmp(&b["date"].as_i64().unwrap())
}

/// sort_list_by_date_reverse
/// 根据日期排序
pub fn sort_list_by_date_reverse(a: &Value, b: &Value) -> std::cmp::Ordering {
    // info!("sort_by {:?},  {:?}", b["date"].as_i64(), a["date"].as_i64());
    b["date"].as_i64().unwrap().cmp(&a["date"].as_i64().unwrap())
}

/// get_device_name
/// get device id by name(space 用户名字或者组名字)
pub async fn get_device_name(stack: &Arc<SharedCyfsStack>, user_name: &str) -> BuckyResult<ObjectId> {
    let cache_key = format!("space.device.{}", user_name);

    // from cache
    let cache_value = CyfsGitCache::get(&cache_key)?;
    if cache_value.is_some() {
        let device_id = cache_value.unwrap();
        info!("get user[{}] device id[{}] from local cache ",user_name, device_id);
        let device = ObjectId::from_str(&device_id).map_err(|e| {
            error!("get user[{}] device id[{}] from local cache failed, invalid objectid ", user_name, device_id);
            BuckyError::new(BuckyErrorCode::InvalidParam, format!("{:?}", e))
        })?;
        return Ok(device);
    }

    let (_, value) = STACK_ACTION.get().unwrap().post_service("owner_id", &json!({"name": user_name}).to_string()).await?;
    // info!("get owner id[{}] from service ", value);
    let value: serde_json::Value = serde_json::from_str(&value).unwrap();
    // check result
    let owner_id = value["data"]["owner_id"].as_str();
    if owner_id.is_none()  {
        error!("post service [owner_id] response error {:?}",value);
        let msg = value["msg"].as_str().unwrap();
        return Err(BuckyError::new(BuckyErrorCode::Failed, msg))
    }

    let target_owner_id = owner_id.unwrap().to_string();
    info!("get owner id[{}] from service ", target_owner_id);
    let target_owner_id = ObjectId::from_str(&target_owner_id).unwrap();
    let device = get_target_device(stack, target_owner_id).await?;
    info!("user[{:?}]'s  target device [{:?}] ", target_owner_id.to_string(), device);

    // set cache
    let _ = CyfsGitCache::put(&cache_key, &device.to_string());

    Ok(device)
}

/// get_device_by_org_name
/// get device id by 组织的名字
pub async fn get_organization_by_name(_stack: &Arc<SharedCyfsStack>, name: &str) -> BuckyResult<serde_json::Value> {
    let cache_key = format!("org_device.{}", name);
    if let Some(cache_value) = CyfsGitCache::get(&cache_key)? {
        return Ok(serde_json::from_str(&cache_value).unwrap())
    }

    let (_, value) = STACK_ACTION.get().unwrap().post_service("organization/owner", &json!({"name": name}).to_string()).await?;
    let value: serde_json::Value = serde_json::from_str(&value).unwrap();
    info!("get org's DID[{}] from service ", value);
    let org_value = value["data"].to_string();
    // info!("get owner id[{}] from service ", target_owner_id);
    // let target_owner_id = ObjectId::from_str(&target_owner_id).unwrap();
    // let device = get_target_device(stack, target_owner_id).await?;
    // info!("user[{:?}]'s  target device [{:?}] ", target_owner_id.to_string(), device);

    // set cache
    let _ = CyfsGitCache::put(&cache_key, &org_value);
    // let org: serde_json::Value = serde_json::from_str(&org_value).unwrap();

    Ok(serde_json::from_str(&org_value).unwrap())
}


/// get_user_name_by_owner
/// get user name by owner id
pub async fn get_user_name_by_owner(user_id: &str) -> BuckyResult<String>  {
    let cache_key = format!("user_name.{}", user_id);

    let cache_value = CyfsGitCache::get(&cache_key)?;
    if cache_value.is_some() {
        let user_name = cache_value.unwrap();
        info!("get user name[{}] from local cache ", user_name);
        return Ok(user_name)
    }
    // 获取 成员名字
    let (_, value) = STACK_ACTION.get().unwrap().post_service("user/name", &json!({"user_id": user_id}).to_string()).await.map_err(|e| {
        error!("post to dec service failed, {:?}", e);
        e
    })?;

    let value: serde_json::Value = serde_json::from_str(&value).unwrap();
    let user_name = value["data"]["name"].as_str().unwrap().to_string();
    info!("get user name[{}  {}] from service ", user_id, user_name);

    // set cache
    let _ = CyfsGitCache::put(&cache_key, &user_name);
    Ok(user_name)
}

pub async fn is_current_device(stack: &Arc<SharedCyfsStack>, device_id: &str) -> BuckyResult<bool> {
    let current_device_id = get_local_device(stack);
    // current_device_id.eq(device_id);
    Ok(current_device_id.to_string() == device_id.to_string())
}



/// check_space
/// 检查space对应device是哪个ood
/// space 是用户名或者org
pub async fn check_space_local(stack: &Arc<SharedCyfsStack>, space: &str)-> BuckyResult<bool> {
    let device = get_device_name(stack, &space).await?;
    let local_device = get_local_device(stack);
    if device.eq(&local_device) {
        info!("target space is local");
        return Ok(true)
    }
    info!("space({}) is no local: target {}, local {}",space, device, local_device);
    Ok(false)
}

/// request_other_ood
pub async fn request_other_ood(stack: &Arc<SharedCyfsStack>,space:&str, route: &str, req_body: &str)-> BuckyResult<NONPostObjectInputResponse> {
    info!("route[{}] target space[{:?}] is other ood", route, space);
    let device = get_device_name(stack, &space).await?;

    let req_path = Some(format!("{}/api", DEC_APP_HANDLER));
    let (_, value) = post_target(stack, device, route, req_body, req_path).await?;
    return Ok(success_proxy(value))
}
