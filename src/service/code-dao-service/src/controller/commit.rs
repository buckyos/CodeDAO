use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

// use std::str::FromStr;
// use super::repository::*;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryCommits {
    author_name: String,
    name: String,
    branch: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryCommit {
    author_name: String,
    name: String,
    // branch: String,
    commit_id: String,
}

/// # repository_commits
/// repo 某个branch的commits
pub async fn repository_commits(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryCommits = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    let repository =
        RepositoryHelper::get_repository_object(&ctx.stack, &data.author_name, &data.name).await?;
    // let commit_object_path = RepositoryHelper::commit_object_map_path(&data.author_name, &data.name);
    let env = ctx.stack_env().await?;

    let commits = git_commits(repository.repo_dir(), &data.branch)?;

    let mut result_data: Vec<serde_json::Value> = Vec::new();
    for item in commits {
        let key = commit_object_map_key(&data.author_name, &data.name, &item.object_id);
        info!("commit full path {}", key);
        let commit_object_id = env.get_by_path(&key).await?;
        if commit_object_id.is_none() {
            continue;
        }
        let buf = get_object(&ctx.stack, commit_object_id.unwrap()).await?;
        let commit = Commit::clone_from_slice(&buf)? as Commit;
        result_data.push(json!({
            "id": commit.id(),
            "oid": commit.object_id(),
            "object_id": commit.object_id(),
            "parent": commit.parent(),
            "tree": commit.tree_oid(),
            "payload": commit.payload(),
            "message": commit.payload(),
            "author": commit.author(),
            "committer": commit.committer(),
        }));
    }
    // loop {
    //     let ret = env.next(10).await?;
    //     if ret.len() == 0 {
    //         break;
    //     }
    //     for item in ret {
    //         // println!("item: {:?}", item);
    //         let (oid, _) = item.into_map_item();
    //         let commit_id = env.get_by_key(oid).await?;
    //         if commit_id.is_some() {
    //             let buf = get_object(stack, commit_id.unwrap()).await?;
    //             let commit = Commit::clone_from_slice(&buf)? as Commit;
    //             data.push(json!({
    //                 "id": commit.id(),
    //                 "oid": commit.object_id(),
    //                 "object_id": commit.object_id(),
    //                 "parent": commit.parent(),
    //                 "tree": commit.tree(),
    //                 "payload": commit.payload(),
    //                 "message": commit.payload(),
    //                 "author": commit.author(),
    //                 "committer": commit.committer(),
    //             }));
    //         }
    //     }
    // }

    Ok(success(json!({ "data": result_data })))
}

/// # repository_commit
/// commit 详细信息
pub async fn repository_commit(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryCommit = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    let repository =
        RepositoryHelper::get_repository_object(&ctx.stack, &data.author_name, &data.name).await?;

    let env = ctx.stack_env().await?;
    let commit_object_path = commit_object_map_key(&data.author_name, &data.name, &data.commit_id);

    let result = env.get_by_path(commit_object_path).await?;
    if result.is_none() {
        return Ok(failed("没有找到对应的 commit"));
    }

    let commit_id = result.unwrap();
    let buf = get_object(&ctx.stack, commit_id).await?;
    let commit = Commit::clone_from_slice(&buf)? as Commit;

    info!(
        "git_show_commit_diff {:?} {:?}",
        commit.parent(),
        commit.object_id()
    );

    let diff = git_show_commit_diff(repository.repo_dir(), commit.parent(), commit.object_id())?;

    Ok(success(json!({"data": {
        "header_info": {
            "id": commit.id(),
            "oid": commit.object_id(),
            "object_id": commit.object_id(),
            "parent": commit.parent(),
            "tree": commit.tree_oid(),
            "payload": commit.payload(),
            "message": commit.payload(),
            "author": commit.author(),
            "committer": commit.committer(),
        },
        "diff": diff,
    }})))
}
