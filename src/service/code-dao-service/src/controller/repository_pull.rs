
use cyfs_lib::*;
use cyfs_base::*;
use log::*;
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::str::FromStr;
use cyfs_git_base::*;
use std::path::{PathBuf};


#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryFetchHead {
    pub name: String,
    pub author_name: String,
    pub is_clone: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryFetch {
    pub name: String,
    pub author_name: String,
    // pub hash: String,
    pub local_hash: String,
    pub branch: String,
}

/// # repository_fetch_head    
/// git pull/fetch head
pub async fn repository_fetch_head(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryFetchHead = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    if repository.init() == 0 {
        return Ok(success(json!({"refs": []})))
    }

    let mut resp_refs =RepositoryBranch::read_refs(&ctx.stack, &space, &name).await?;

    // clone 的时候， 要返回 default branch 
    // 而且仅能返回一个参数
    // TODO opt
    if data.is_clone {
        resp_refs = resp_refs.drain(..).filter(|repo_ref| {
            &repo_ref.branch == "master"
        }).collect::<Vec<GitRef>>();
    }


    Ok(success(json!({"refs": resp_refs})))
}




/// # repository_fetch    
/// git pull/clone
pub async fn repository_fetch(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryFetch = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    if repository.init() == 0 {
        return Ok(failed("current repository no init"))
    }

    let repository_dir = repository.repo_dir();

    // 如果是全量fetch: git rev-list --objects <最新ref-hash>
    // 如果是增量,拿到了local,就调整一下 rev-list的参数  git rev-list --objects  9104c74618190e97a2d013812d0315147477e94e..f4a25662c0f035106f73338fe12ce81295041c8c
    // 9104c74618190e97a2d013812d0315147477e94e..master
    let rev_list_params = if data.local_hash == "" {
        format!("{}", data.branch)
    } else {
        format!("{}..{}", data.local_hash, data.branch)
    };

    info!("{}/{} current fetch rev list params: {}",space, name, rev_list_params);
    let git_objects_content = git_rev_list_objects(repository_dir.clone(), &rev_list_params)?;
    let target_pack_file = git_pack_objects_to_file(repository_dir.clone(), git_objects_content)?;
    let file_id = publish_file(
        &ctx.stack, 
        // local_path,
        PathBuf::from_str(&target_pack_file).unwrap()
    ).await?;
    info!("git fetch: publish_file file id {:?} ", file_id);
    Ok(success(json!({
        "file_id": file_id.to_string(),
        "device_id": get_local_device(&ctx.stack).to_string(),
    })))

}