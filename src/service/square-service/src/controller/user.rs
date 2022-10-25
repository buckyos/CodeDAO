use cyfs_lib::*;
use cyfs_base::*;
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;
//use sqlx::Row;
use cyfs_git_base::*;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserGetByName {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserGetNameByUserId {
    user_id: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct RequestUserList {
    #[serde(rename = "user_name")]
    pub search_name: Option<String>,
}



/// # user_list    
/// 用户列表
pub async fn user_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let req_data: RequestUserList = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_single_env().await?;
    let mut data: Vec<serde_json::Value> = Vec::new();
    let result = env.load_by_path(USER_LIST_PATH).await;
    if result.is_err() {
        return Ok(success(json!({"data": []})))
    }

    let ret = env.list().await.unwrap();
    for item in ret {
        let (_,id) = item.into_map_item();
        let value = UserHelper::json(&ctx.stack, id).await;
        if value.is_err() {
            error!("get user info failed {:?}", value.err());
            continue;
        }
        let value = value.unwrap();
        let user_name = value["name"].to_string();
        let is_search_match = if let Some(ref search_name) = req_data.search_name {
            if user_name.contains(search_name) {
                true
            } else {
                false
            }
        } else {
            true
        };
        if is_search_match {
            data.push(json!(value));
        }
    }

    // 需要把vec再转成 Value
    Ok(success(json!({"data": data})))
}


/// # owner_id_by_name    
/// 获取 名字(用户名或者组名) 对应的ownerid
pub async fn owner_id_by_name(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserGetByName = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_env().await?;
    let user_name_key = format!("{}{}", USER_NAME_LIST_PATH, data.name);
    if let Ok(Some(object_id)) = env.get_by_path(user_name_key).await {
        let user = UserHelper::user(&ctx.stack, object_id).await?;
        return Ok(success(json!({
            "owner_id": user.owner(),
            "name": data.name,
            "type": "user",
        })))
    }
    
    // find organization 
    let map_org_key  = format!("{}/{}" ,ORG_LIST_PATH, data.name);
    if let Ok(Some(object_id)) = env.get_by_path(map_org_key).await {
        let buf = get_object(&ctx.stack, object_id).await?;
        let org = Organization::clone_from_slice(&buf)?;
        return Ok(success(json!({
            "owner_id": org.owner(),
            "name": org.name(),
            "type": "org",
        })))
    }
    
    return Ok(failed("space no found"))
}

/// # name_by_owner_id    
/// 
pub async fn name_by_owner_id(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserGetNameByUserId = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_env().await?;
    let map_user_id_key = format!("{}{}", USER_LIST_PATH, data.user_id);
    if let Ok(Some(object_id)) = env.get_by_path(map_user_id_key).await {
        let user = UserHelper::user(&ctx.stack, object_id).await?;
        return Ok(success(json!({
            "owner_id": user.owner(),
            "name": user.name(),
            "type": "user",
        })))
    }

    return Ok(failed("target owner id not fountd"))
}
