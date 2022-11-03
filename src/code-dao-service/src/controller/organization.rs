use cyfs_lib::*;
use cyfs_base::*;
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use cyfs_git_base::*;
use crate::*;


#[derive(Serialize, Deserialize, Debug)]
struct RequestOrganizationNew {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestOrganizationHome {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestOrganizationAddMember {
    name: String,
    user_id: String,
}


pub async fn get_org_by_name(stack: &Arc<SharedCyfsStack>, name: &str) -> BuckyResult<Organization> {
    let org_key  = format!("{}/{}" ,ORG_LIST_PATH, name);
    let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
    let result = env.get_by_path(&org_key).await?;
    if result.is_some() {
        let object_id = result.unwrap();
        let buf = get_object(stack, object_id).await?;
        let org = Organization::clone_from_slice(&buf)?;
        return Ok(org)
    }

    Err(BuckyError::new(BuckyErrorCode::NotFound, format!("organization[{}] not found", name)))
}

/// # organization_new    
/// 创建组织
pub async fn organization_new(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationNew = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    // create org
    let org = Organization::create(
        ctx.caller, 
        data.name, 
        "".to_string(), 
        data.email, 
        "".to_string());


    info!("send organization object to service {}, {}", org.desc().object_id(), org.name());
    post_special_object_service(&ctx.stack, &org, "organization").await?;

    insert_organization(&ctx.stack, &org).await?;

    // 把创建者 也加入 org的成员列表里
    let user_name = get_user_name_by_owner(&ctx.caller.to_string()).await?;
    
    let member = OrganizationMember::create(
        ctx.caller, 
        org.name().to_string(), 
        ctx.caller.to_string(), 
        user_name,
        "admin".to_string(),
    );
    put_object(&ctx.stack, &member).await?;
    insert_organization_member(&ctx.stack, &member).await?;
    
    
    Ok(success(json!({})))
}

/// # organization_list    
/// 组织列表
pub async fn organization_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let (_, resp_body) = STACK_ACTION.get().unwrap().post_service(&ctx.route, &ctx.data).await?;
    Ok(success_proxy(resp_body))
}


/// # organization_home    
/// 组的首页
pub async fn organization_home(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.name).await? {
        return Ok(result);
    }
 
    let value = get_organization_by_name(&ctx.stack, &data.name).await?;

    // let target_owner_id = value["creator"].as_str().unwrap().to_string();

    // info!("get owner id[{}] from service ", target_owner_id);
    // let target_owner_id = ObjectId::from_str(&target_owner_id).unwrap();
    // let device = get_target_device(stack, target_owner_id).await?;
    // info!("user[{:?}]'s  target device [{:?}] ", target_owner_id.to_string(), device);
    // let is_current = is_current_device(stack, &device.to_string()).await?;
    // info!("deivce {:?}, is current device: {}",device, is_current);

    // if !is_current {
    //     // request other
    // }

    let is_author_add = get_owner(&ctx.stack).await.eq(&ctx.caller);


    let env = ctx.stack_single_env().await?;
    let base_path = format!("{}/{}", ORG_MEMBER_PATH, data.name);
    let result = env.load_by_path(base_path).await;
    if result.is_err() {
        info!("empty path");

        return Ok(success(json!({
            "id":  value["org_id"].as_str().unwrap().to_string(),
            "name":  value["name"].as_str().unwrap().to_string(),
            "email": value["email"].as_str().unwrap().to_string(),
            "creator":  value["creator"].as_str().unwrap().to_string(),
            "member_count": 0,
            "repository_count": 0,
            "is_show_add": is_author_add,
        })))
    }
    

    let mut count = 0;
    loop {
        let ret = env.next(10).await?;
        if ret.len() == 0 {
            break;
        }
        for item in ret {
            info!("item: {:?}", item);
            count +=1;
        }
    }


    let repository_count = {
        let mut count = 0;
        let env = ctx.stack_single_env().await?;
        let base_path = format!("{}/{}", ORG_REPO_PATH, data.name);
        if env.load_by_path(base_path).await.is_ok() {
            loop {
                let ret = env.next(10).await?;
                if ret.len() == 0 {
                    break;
                }
                for item in ret {
                    info!("item: {:?}", item);
                    count +=1;
                }
            }
        }
        count
    };



    Ok(success(json!({
        "id":  value["org_id"].as_str().unwrap().to_string(),
        "name":  value["name"].as_str().unwrap().to_string(),
        "email": value["email"].as_str().unwrap().to_string(),
        "creator":  value["creator"].as_i64().unwrap().to_string(),
        "member_count": count,
        "repository_count": repository_count,
        "is_show_add": is_author_add,
    })))
}


pub async fn organization_member_list(stack: &Arc<SharedCyfsStack>, org_name: &str) -> BuckyResult<Vec<serde_json::Value>> {
    let env = stack.root_state_stub(None, Some(dec_id())).create_single_op_env().await?;
    let base_path = format!("{}/{}", ORG_MEMBER_PATH, org_name);
    env.load_by_path(base_path).await?;
    let mut members:Vec<serde_json::Value> = Vec::new();

    let ret = env.list().await?;
    for item in ret {
	info!("item: {:?}", item);
	let (_, id) = item.into_map_item();
	let buf = get_object(&stack, id).await?;
	let member = OrganizationMember::clone_from_slice(&buf)?;
	members.push(json!({
	    "id":  member.id(),
	    "user_name":  member.user_name(),
	    "user_id":  member.user_id(),
	    "role":  member.role(),
	    "date": member.date(),
	}));
    }

    Ok(members)
}


/// # organization_member
/// 组的的成员
pub async fn organization_member(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let is_local = check_space_local(&ctx.stack, &data.name).await?;
    if !is_local {
        return request_other_ood(&ctx.stack, &data.name, "organization/member", &ctx.data).await
    }

    let _value = get_organization_by_name(&ctx.stack, &data.name).await?;

    // let target_owner_id = value["creator"].as_str().unwrap().to_string();

    // info!("get owner id[{}] from service ", target_owner_id);
    // let target_owner_id = ObjectId::from_str(&target_owner_id).unwrap();
    // let device = get_target_device(stack, target_owner_id).await?;
    // info!("user[{:?}]'s  target device [{:?}] ", target_owner_id.to_string(), device);
    // let is_current = is_current_device(stack, &device.to_string()).await?;
    // info!("deivce {:?}, is current device: {}",device, is_current);
    let mut members:Vec<serde_json::Value> = organization_member_list(&ctx.stack, &data.name).await?;
    members.sort_by(sort_list_by_date);

    Ok(success(json!({
        "data": members
    })))
}

///organization_repository
/// 
pub async fn organization_repository(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let is_local = check_space_local(&ctx.stack, &data.name).await?;
    if !is_local {
        return request_other_ood(&ctx.stack, &data.name, "organization/repository", &ctx.data).await
    }

    let _value = get_organization_by_name(&ctx.stack, &data.name).await?;

    // let target_owner_id = value["creator"].as_str().unwrap().to_string();

    // info!("get owner id[{}] from service ", target_owner_id);
    // let target_owner_id = ObjectId::from_str(&target_owner_id).unwrap();
    // let device = get_target_device(stack, target_owner_id).await?;
    // info!("user[{:?}]'s  target device [{:?}] ", target_owner_id.to_string(), device);
    // let is_current = is_current_device(stack, &device.to_string()).await?;
    // info!("deivce {:?}, is current device: {}",device, is_current);


    let env = ctx.stack_single_env().await?;
    let base_path = format!("{}/{}", ORG_REPO_PATH, data.name);
    env.load_by_path(base_path).await?;
    let mut reponse_data:Vec<serde_json::Value> = Vec::new();
    loop {
        let ret = env.next(10).await?;
        if ret.len() == 0 {
            break;
        }
        for item in ret {
            info!("item: {:?}", item);
            let (_, id) = item.into_map_item();
            let buf = get_object(&ctx.stack, id).await?;
            let repo = Repository::clone_from_slice(&buf)?;
            reponse_data.push(json!({
                "id":  repo.id(),
                "author_name":  repo.author_name(),
                "name":  repo.name(),
                "description":  repo.description(),
                "is_private": repo.is_private(),
                "date":  repo.date(),
            }));
        }
    }

    Ok(success(json!({
        "data": reponse_data
    })))
}



///organization_member_add
/// 组的的成员
pub async fn organization_member_add(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationAddMember = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let owner = get_owner(&ctx.stack).await;


    // 把创建者 也加入 org的成员列表里
    let user_name = get_user_name_by_owner(&data.user_id).await?;

    let member = OrganizationMember::create(
        owner, 
        data.name.clone(), 
        data.user_id.clone(), 
        user_name.clone(),
        "member".to_string(),
    );
    put_object(&ctx.stack, &member).await?;
    info!("put new memeber[{}] object: {} ", member.user_name(), member.id());

    insert_organization_member(&ctx.stack, &member).await?;
    info!("add new memeber[{}] ok ", member.user_name());



    // org 对象推送给 新成员
    let org = get_org_by_name(&ctx.stack, &data.name).await?;
    let device = get_device_name(&ctx.stack, &user_name).await?;
    //let _r = put_object_target(&ctx.stack, &org, Some(device), Some("organization/add".to_string())).await?;
    post_special_object_target_ood(&ctx.stack, &org, Some(device), "organization").await?;

    Ok(success(json!({
        "message": "ok"
    })))
}
