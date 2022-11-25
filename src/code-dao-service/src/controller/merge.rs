use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeCompare {
    author_name: String,
    name: String,
    // 目标分支
    target: String,
    // 源分支
    origin: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeCreate {
    author_name: String,
    name: String,
    // 目标分支
    target: String,
    // 源分支
    origin: String,
    title: String,
    merge_type: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeList {
    author_name: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeDetail {
    merge_id: String, //这个是path上的id
    author_name: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeAccept {
    merge_id: String, //这个是path上的id
    author_name: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryMergeCompareFile {
    author_name: String,
    name: String,
    target: String,
    origin: String,
    file_name: String,
}

/// # repository_merge_compare   
/// 合并 比对分支差异情况
pub async fn repository_merge_compare(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeCompare =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;

    if repository.init() == 0 {
        return Ok(failed("current repository no init"));
    }
    let repository_dir = repository.repo_dir();

    info!("to get compare result");
    let result = git_compare_no_merge(repository_dir.clone(), &data.origin, &data.target)?;
    info!("git compare result {:?}", result);

    let diff_result = git_diff_stat(repository_dir, &data.origin, &data.target)?;
    info!("git compare result {:?}", diff_result);

    Ok(success(json!({
        "data": {
            "commits": result,
            "diff": diff_result,
        }
    })))
}

/// # repository_merge_create
/// 创建合并请求
pub async fn repository_merge_create(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeCreate =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    // 获取 用户名字
    let user_name = get_user_name_by_owner(&ctx.caller.to_string()).await?;
    info!("get user name[{}  {}] ", &ctx.caller.to_string(), user_name);
    let owner = get_owner(&ctx.stack).await;

    // 创建 merge request 对象
    let merge = MergeRequest::create(
        owner,
        data.title,
        data.origin,
        data.target,
        data.merge_type,
        "open".to_string(),
        data.author_name.clone(),
        data.name.clone(),
        user_name,
    );

    put_object(&ctx.stack, &merge).await?;
    let env = ctx.stack_env().await?;
    let object_id = merge.desc().calculate_id();

    let merge_key =
        RepositoryHelper::merge_key(&data.author_name, &data.name, &object_id.to_string());
    let r = env.insert_with_path(&merge_key, &object_id).await?;
    info!("insert_with_key: {:?}", r);

    let _root = env.commit().await;

    Ok(success(json!({"message": "ok"})))
}

/// # repository_merge_list
/// 合并请求的列表
pub async fn repository_merge_list(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeList =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    info!("get repository merge list");

    let env = ctx.stack_single_env().await?;
    let mut response_data: Vec<serde_json::Value> = Vec::new();
    let base_key = RepositoryHelper::merge_base(&space, &name);

    if env.load_by_path(&base_key).await.is_ok() {
        let merge_list = env.list().await?;
        for item in merge_list {
            println!("item: {:?}", item);
            let (key, id) = item.into_map_item();
            // 现在只能逐个去拿
            let buf = get_object(&ctx.stack, id).await?;
            let merge = MergeRequest::clone_from_slice(&buf)?;
            // let issue = RepositoryHelper::issue_by_path(stack, &data.author_name, &data.name, &issue_id, "0").await?;
            response_data.push(json!({
                // "id":  merge.id(),
                "id": key,
                "title": merge.title(),
                "status": merge.status(),
                "user_name": merge.user_name(),
                "origin_branch": merge.origin_branch(),
                "target_branch": merge.target_branch(),
                "merge_type": merge.merge_type(),
            }));
        }
    }

    Ok(success(json!({ "data": response_data })))
}

/// # repository_merge_detail
/// 合并请求的详情
pub async fn repository_merge_detail(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeDetail =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let env = ctx.stack_env().await?;
    let merge_key = RepositoryHelper::merge_key(&space, &name, &data.merge_id);
    info!("merge/detail merge key {}", merge_key);
    let result = env.get_by_path(merge_key).await?;
    if result.is_none() {
        return Ok(success(json!({"message": "not found"})));
    }

    let buf = get_object(&ctx.stack, result.unwrap()).await?;
    let merge = MergeRequest::clone_from_slice(&buf)?;

    Ok(success(json!({"data": {
        "id":  merge.id(),
        "title": merge.title(),
        "status": merge.status(),
        "user_name": merge.user_name(),
        "origin_branch": merge.origin_branch(),
        "target_branch": merge.target_branch(),
        "merge_type": merge.merge_type(),
        "author_name": merge.repository_author_name(),
        "name": merge.repository_name(),
        "date": merge.date(),
    }})))
}

/// # repository_merge_accept
/// 处理合并请求
pub async fn repository_merge_accept(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeAccept =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    // 通过path获取 merge object
    let env = ctx.stack_env().await?;
    let merge_key = RepositoryHelper::merge_key(&space, &name, &data.merge_id);
    let result = env.get_by_path(merge_key.clone()).await?;
    if result.is_none() {
        return Ok(success(json!({"message": "not found"})));
    }
    let buf = get_object(&ctx.stack, result.unwrap()).await?;
    let merge = MergeRequest::clone_from_slice(&buf)?;

    // 获取 用户名字
    // let user_name = get_user_name_by_owner(&&ctx.caller.to_string()).await?;

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    let repository_dir = repository.repo_dir();

    // set user info
    let user = UserHelper::get_current_user(&ctx.stack).await?;
    git_exec_base(
        repository_dir.clone(),
        ["config", "user.email", user.email()],
    )?;
    git_exec_base(repository_dir.clone(), ["config", "user.name", user.name()])?;
    // TODO? check: 是否在同一个分支线上。

    // remove index file (staged)
    // TODO 并发的时候要lock这个文件的处理，现在先这样吧
    let _ = std::fs::remove_file(repository_dir.clone().join(".git/index"));

    let new_hash = git_merge(
        repository_dir.clone(),
        merge.origin_branch(),
        merge.target_branch(),
    )?;
    info!(
        "git_merge[{}] result new_hash: {:?}",
        merge.target_branch(),
        new_hash
    );

    let commit = git_read_commit_object(repository_dir, &new_hash)?;
    info!("git_read the new merged commit: {:?}", commit);

    let commit_object = Commit::create(
        ctx.caller,
        commit.object_id.clone(),
        vec![commit.parent, commit.parent2],
        commit.tree.clone(),
        commit.payload,
        Some(CommitSignature {
            name: commit.author.name,
            email: commit.author.email,
            when: commit.author.date,
        }),
        Some(CommitSignature {
            name: commit.committer.name,
            email: commit.committer.email,
            when: commit.committer.date,
        }),
    );
    let commit_object_id = commit_object.desc().object_id();
    put_object(&ctx.stack, &commit_object).await?;

    info!("commit_obj object id: {:?}", commit_object_id);
    let env = ctx.stack_env().await?;
    let commit_path = RepositoryHelper::commit_object_map_path(&space, &name);
    let _r = env
        .set_with_key(
            &commit_path,
            &commit.object_id,
            &commit_object_id,
            None,
            true,
        )
        .await?;
    let root = env.commit().await;
    info!(
        "repository_merge_accept after create commit object dec root is: {:?}",
        root
    );

    // update-ref
    let repo_ref = RepositoryBranch::create(
        get_owner(&ctx.stack).await,
        space.clone(),
        name.clone(),
        merge.target_branch().to_string().clone(),
        new_hash.clone(),
    );
    repo_ref.insert_ref(&ctx.stack).await?;

    // change status
    let new_merge = MergeRequest::create(
        merge.desc().owner().unwrap(),
        merge.title().to_string(),
        merge.origin_branch().to_string(),
        merge.target_branch().to_string(),
        merge.merge_type().to_string(),
        "close".to_string(),
        merge.repository_author_name().to_string(),
        merge.repository_name().to_string(),
        merge.user_name().to_string(),
    );
    put_object(&ctx.stack, &new_merge).await?;
    let new_merge_id = new_merge.desc().calculate_id();

    // 可能可以优化这个 env
    let env = ctx.stack_env().await?;
    let _r = env
        .set_with_path(merge_key, &new_merge_id, None, true)
        .await?;
    let _root = env.commit().await;

    Ok(success(json!({"data": "ok"})))
}

/// # repository_merge_compare_file
/// 合并请求的文件(逐个)比对
pub async fn repository_merge_compare_file(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryMergeCompareFile =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    // TODO no suport
    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    let repository_dir = repository.repo_dir();
    let result = git_merge_diff_file(repository_dir, &data.origin, &data.target, &data.file_name)?;

    Ok(success(json!({
        "data": {
            "not_support": false,
            "content": [
                {
                    "file_content": result
                }
            ]
        }
    })))
}

/// # panel_merge_list
/// 全局merge列表
pub async fn panel_merge_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let env = ctx.stack_single_env().await?;
    let result = env.load_by_path(REPOSITORY_PATH).await;
    if result.is_err() {
        return Ok(success(json!({
            "data": {
                "merge_list": []
            }
        })));
    }

    let mut response_data: Vec<serde_json::Value> = Vec::new();

    let mut data: Vec<(String, String)> = Vec::new();
    let ret = env.list().await?;
    for item in ret {
        let (space, space_object_id) = item.into_map_item();
        let sub_env = ctx.stack_single_env().await?;
        sub_env.load(space_object_id).await?;
        let sub_ret = sub_env.list().await.unwrap();
        for sub_item in sub_ret {
            let (name, _) = sub_item.into_map_item();
            data.push((space.clone(), name));
        }
    }

    for repo in data {
        let key = RepositoryHelper::merge_base(&repo.0, &repo.1);
        let env = ctx.stack_single_env().await?;
        if env.load_by_path(key).await.is_ok() {
            let ret = env.list().await?;
            for item in ret {
                let (id, _) = item.into_map_item();
                let object_id = ObjectId::from_str(&id)?;
                let buf = get_object(&ctx.stack, object_id).await?;
                let merge = MergeRequest::clone_from_slice(&buf)?;
                response_data.push(json!({
                    "id": merge.id(),
                    "title": merge.title(),
                    "status": merge.status(),
                    "author_name": merge.repository_author_name(),
                    "name": merge.repository_name(),
                    "user_name": merge.user_name(),
                    "date": merge.date(),
                }));
            }
        };
    }

    Ok(success(json!({
        "data":  {
            "merge_list": response_data
        }
    })))
}
