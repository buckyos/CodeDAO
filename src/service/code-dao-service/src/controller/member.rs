use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMemberAdd {
    author_name: String,
    name: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMemberList {
    author_name: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMemberDelete {
    author_name: String,
    name: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMemberRole {
    author_name: String,
    name: String,
    user_id: String,
    role: String,
}

pub async fn member_list(
    stack: &Arc<SharedCyfsStack>,
    space: &str,
    name: &str,
) -> BuckyResult<Vec<serde_json::Value>> {
    let env = stack
        .root_state_stub(None, Some(dec_id()))
        .create_single_op_env()
        .await?;
    let base_key = RepositoryHelper::member_base_path(space, name);
    let mut response_data: Vec<serde_json::Value> = Vec::new();

    if env.load_by_path(&base_key).await.is_ok() {
        let ret = env.list().await?;
        for item in ret {
            info!("member_list next item: {:?}", item);
            let (member_object_id, _) = item.into_map_item();
            // 现在只能逐个去拿
            let member =
                RepositoryHelper::member_by_path(stack, space, name, &member_object_id).await?;
            response_data.push(json!({
                "id": member.id(),
                "user_id": member.user_id(),
                "user_name": member.user_name(),
                "role": member.role(),
            }));
        }
    }
    Ok(response_data)
}

/// # repository_member_list   
/// 仓库成员列表
pub async fn repository_member_list(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMemberList =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let member_list = member_list(&ctx.stack, &space, &name).await?;

    Ok(success(json!({ "data": member_list })))
}

/// # repository_member_add
/// 添加成员
pub async fn repository_member_add(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMemberAdd =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    // check permission
    let env = ctx.stack_env().await?;

    let repo_member_key =
        RepositoryHelper::member_key(&data.author_name, &data.name, &data.user_id);
    let result = env.get_by_path(&repo_member_key).await?;
    if result.is_some() {
        return Ok(failed(&format!(
            "member[{}] was already add",
            &data.user_id
        )));
    }

    let owner = get_owner(&ctx.stack).await;

    // 获取 成员名字
    let user_name = get_user_name_by_owner(&data.user_id).await?;
    info!("get user name[{}  {}] ", &data.user_id, user_name);

    //  添加到 objectmap
    let member = RepositoryMember::create(
        owner,
        data.user_id,
        "".to_string(),
        RepositoryMemberRole::Read,
        user_name.clone(),
        data.author_name.clone(),
        data.name.clone(),
    );
    put_object(&ctx.stack, &member).await?;

    let object_id = member.desc().calculate_id();
    let _r = env
        .set_with_path(&repo_member_key, &object_id, None, true)
        .await?;

    // 投递repo object to target ood
    let device = get_device_name(&ctx.stack, &user_name).await?;
    info!("get target member 's device {:?}", device);

    // commit objectmap 放在put target后面，put target失败了，就不会commit
    let repository =
        RepositoryHelper::get_repository_object(&ctx.stack, &data.author_name, &data.name).await?;
    info!("put repo[{}] to target device ", repository.name());    
    post_special_object_target_ood(&ctx.stack, &repository, Some(device), "repository").await?;


    let root = env.commit().await;
    info!("insert repo member root: {:?}", root);

    Ok(success(json!({})))
}

/// # repository_member_delete    
/// 删除仓库成员
pub async fn repository_member_delete(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMemberDelete =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;

    // TODO check permission

    let env = ctx.stack_env().await?;
    let repo_member_key =
        RepositoryHelper::member_key(&data.author_name, &data.name, &data.user_id);

    // check key exist
    let result = env.get_by_path(&repo_member_key).await?;
    if result.is_none() {
        return Ok(failed(&format!(
            "repository member [{}] not exist",
            data.user_id
        )));
    }

    let member =
        RepositoryHelper::member_by_path(&ctx.stack, &data.author_name, &data.name, &data.user_id)
            .await?;

    let _r = env.remove_with_path(&repo_member_key, None).await?;

    // remote delete
    let device = get_device_name(&ctx.stack, member.user_name()).await?;
    let req_path = Some(format!("{}/api", DEC_APP_HANDLER));
    post_target(
        &ctx.stack,
        device,
        "remote/repo/delete",
        &ctx.data,
        req_path,
    )
    .await?;

    // 重复的去删除 创建，  commit可能会失败
    let root = env.commit().await;
    info!("remove commit: {:?}", root);

    Ok(success(json!({"message": "ok"})))
}

/// # repository_member_role    
/// 更改仓库成员角色
pub async fn repository_member_role(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMemberRole =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;

    // // TODO check permission

    let env = ctx.stack_env().await?;
    let repo_member_key =
        RepositoryHelper::member_key(&data.author_name, &data.name, &data.user_id);
    let result = env.get_by_path(&repo_member_key).await?;
    if result.is_none() {
        return Ok(failed(&format!(
            "repository member [{}] not exist",
            data.user_id
        )));
    }

    // 获取旧的object
    let buf = get_object(&ctx.stack, result.unwrap()).await?;
    let prev_member = RepositoryMember::clone_from_slice(&buf)?;

    info!(
        "repo member role change  [{}] -> [{}]",
        prev_member.role(),
        data.role
    );

    let member = RepositoryMember::create(
        prev_member.desc().owner().unwrap(),
        prev_member.user_id().to_string(),
        prev_member.repository_id().to_string(),
        RepositoryMemberRole::from_str(&data.role)?,
        prev_member.user_name().to_string(),
        prev_member.repository_author_name().to_string(),
        prev_member.repository_name().to_string(),
    );
    put_object(&ctx.stack, &member).await?;
    let object_id = member.desc().calculate_id();

    let r = env.remove_with_path(&repo_member_key, None).await?;
    println!("remove_with_key: {:?}", r);
    let r = env
        .set_with_path(&repo_member_key, &object_id, None, true)
        .await?;
    println!("insert_with_key: {:?}", r);

    let root = env.commit().await;
    println!("new dec root is: {:?}", root);
    Ok(success(json!({"message": "ok"})))
}
