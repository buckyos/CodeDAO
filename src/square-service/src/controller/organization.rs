use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct RequestOrganizationHome {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestOrganizationList {
    #[serde(rename = "organization_name")]
    pub search_name: Option<String>,
}

/// # organization_list    
/// 组织列表
pub async fn organization_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let req_data: RequestOrganizationList =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_single_env().await?;
    let mut data: Vec<serde_json::Value> = Vec::new();

    if let Ok(_result) = env.load_by_path(ORG_LIST_PATH).await {
        let ret = env.list().await.unwrap();
        for item in ret {
            info!("item: {:?}", item);
            let (_, id) = item.into_map_item();
            let buf = get_object(&ctx.stack, id).await?;
            let org = Organization::clone_from_slice(&buf)?;

            let is_search_match = if let Some(ref search_name) = req_data.search_name {
                if org.name().contains(search_name) {
                    true
                } else {
                    false
                }
            } else {
                true
            };
            if is_search_match {
                data.push(json!({
                    "id": org.id(),
                    "date": org.date(),
                    "name": org.name(),
                    "email": org.email(),
                }));
            }
        }
    }

    // 需要把vec再转成 Value
    Ok(success(json!({
        "data": data,
        "count": data.len(),
    })))
}

/// # organization_get_owner_by_name    
///
pub async fn organization_get_owner_by_name(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestOrganizationHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let env = ctx.stack_env().await?;

    // find organization
    let map_org_key = format!("{}/{}", ORG_LIST_PATH, data.name);
    if let Ok(Some(object_id)) = env.get_by_path(map_org_key).await {
        let buf = get_object(&ctx.stack, object_id).await?;
        let org = Organization::clone_from_slice(&buf)?;
        return Ok(success(json!({
            "owner_id": org.owner(),
            "org_id": org.id(),
            "name": org.name(),
            "email": org.email(),
            "avatar": org.avatar(),
            "description": org.description(),
            "creator": org.date(),
        })));
    }

    let msg = format!("target organization[{}] no found", data.name);
    info!("{}", msg);
    return Ok(failed(&msg));
}
