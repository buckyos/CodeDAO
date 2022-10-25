use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryDelete {
    // owner: Option<String>,
    pub name: String,
    pub author_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryHome {
    pub name: String,
    pub author_name: String,
    pub branch: String,
}

type RequestRepositoryFind = RequestRepositoryDelete;


#[derive(Serialize, Deserialize, Debug)]
struct RequestRepositoryList {
    #[serde(rename = "repo_name")]
    pub search_name: Option<String>,
}


/// # repository_list    
/// 仓库列表
pub async fn repository_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let req_data: RequestRepositoryList = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_single_env().await?;
    let result = env.load_by_path(REPOSITORY_PATH).await;
    if result.is_err() {
        return Ok(success(json!({"data": []})));
    }

    let mut data: Vec<serde_json::Value> = Vec::new();
    let ret = env.list().await?;
    for item in ret {
        let (space, space_object_id) = item.into_map_item();
        let sub_env = ctx.stack_single_env().await?;
        sub_env.load(space_object_id).await?;
        let result = sub_env.list().await?;
        for sub_item in result {
            //println!("sub_item: {:?}", sub_item);
            let (repository_name, _) = sub_item.into_map_item();
            info!("read repository [{}/{}] object", space, repository_name);
            let repository =
                RepositoryHelper::get_repository_object(&ctx.stack, &space, &repository_name).await;
            if repository.is_ok() {
                let repository = repository.unwrap();
                let is_search_match = if let Some(ref search_name) = req_data.search_name {
                    if repository.name().contains(search_name) {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };
                if is_search_match {
                    data.push(repository.json());
                }


            } else {
                error!("read repository object failed {:?}", repository.err());
            }
        }
    }

    Ok(success(json!({ "data": data })))
}

/// # repository_find    
/// 查找仓库 （用于 remote模块）
pub async fn repository_find(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryFind = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    let helper = ctx.repository_helper(space.clone(), name.clone());
    let repository = helper.repository().await?;
    let repository_value = repository.json();

    // TODO  other git infomation
    Ok(success(json!({ "repository": repository_value })))
}

/// # repository_delete    
/// 删除仓库
pub async fn repository_delete(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryDelete = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let env = ctx.stack_env().await?;

    let (repository_key, _) = RepositoryHelper::object_map_path(&space, &name);

    // check key exist
    let result = env.get_by_path(&repository_key).await?;
    if result.is_none() {
        return Ok(failed(&format!(
            "repository[{}/{}] not exist",
            &space, &name
        )));
    }

    // TODO check permission
    let object_id = result.unwrap();
    // let meta = env.metadata();
    info!("result: {:?}", object_id);

    let _r = env
        .remove_with_path(&repository_key, Some(object_id))
        .await?;
    let root = env.commit().await;
    info!("remove commit: {:?}", root);

    delete_object(&ctx.stack, object_id).await?;
    Ok(success(json!({"message": "ok"})))
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use std::str::FromStr;
    pub const DEC_ID: &'static str = "9tGpLNnSx4GVQTqg5uzUucPbK1TNJdZk3nNA77PPJaPW";
    #[async_std::test]
    async fn test_repo_delete() {
        let o = ObjectId::from_str(DEC_ID).unwrap();
        let stack = Arc::new(SharedCyfsStack::open_default(Some(o)).await.unwrap());
        stack.wait_online(None).await.unwrap();
        let env = stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await
            .unwrap();
        let r = env
            .remove_with_key("/app/repo/sunxinle/aaa", "commit", None)
            .await
            .unwrap();
        println!("remove_with_path: {:?}", r);

        let root = env.commit().await.unwrap();
        println!("remove_with_path commit: {:?}", root);
    }
}
