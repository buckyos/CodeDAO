use super::controller::*;
use async_std::sync::Arc;
use async_trait::async_trait;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use cyfs_util::*;
use log::*;
use serde_json::json;
use crate::put_object::*;

pub struct OnCommonPostHandle {
    pub stack: Arc<SharedCyfsStack>,
    pub put_object_helper: PutObjectHelper,
}

#[async_trait]
impl EventListenerAsyncRoutine<RouterHandlerPostObjectRequest, RouterHandlerPostObjectResult>
    for OnCommonPostHandle
{
    async fn call(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<RouterHandlerPostObjectResult> {
        info!(
            "service handler: req_path {:?}",
            param.request.common.req_path
        );

        if let Some(req_path) = &param.request.common.req_path {
            let req_path = RequestGlobalStatePath::parse(req_path)?.req_path.unwrap();
            let mut req_path_info = req_path.trim_start_matches('/').split('/');
            let req_path_type = req_path_info.nth(1);
            info!("app handler: req_path type {:?}", req_path_type);
	    
            if let Some("object") = req_path_type {
                let object_type = req_path_info.next().unwrap();
                info!(
                    "service handler: req_path is a other object: {}",
                    object_type
                );
                let resp = match object_type {
                    "user" => self.put_object_helper.user_init(param).await,
                    "repository" => self.put_object_helper.repository_new(param).await,
                    "organization" => self.put_object_helper.organization_new(param).await,
                    _ => {
                        let msg = "req_path object type no match any object";
                        error!("{}", msg);
                        Ok(failed(msg))
                    }
                };
                if resp.is_err() {
                    error!("[{}] error: {:?}", req_path, resp.as_ref().err()); 
                }

                return Ok(RouterHandlerPostObjectResult {
                    action: RouterHandlerAction::Response,
                    request: None,
                    response: Some(resp),
                });
            }
        }

        let ctx = PostContext::new_by_param(param, self.stack.clone());
        info!(
            "[{}] request route[{:?}], param:{:?}, caller:{:?}",
            ctx.id, ctx.route, ctx.data, ctx.caller
        );
        let ctx1 = ctx.clone();

        // dec service route
        // route match
        let post_object_response = match ctx.route.as_str() {
            "user/list" => user_list(ctx).await,
            "owner_id" => owner_id_by_name(ctx).await,
            "user/name" => name_by_owner_id(ctx).await,
            "organization/list" => organization_list(ctx).await,
            "organization/owner" => organization_get_owner_by_name(ctx).await,
            "repo/list" => repository_list(ctx).await,
            "repo/delete" => repository_delete(ctx).await,
            "repo/find" => repository_find(ctx).await,
            _ => {
                error!("not found route: {}", ctx.route);
                Ok(success(json!({"msg": "route not found"})))
            }
        }
        .or_else(|e| -> BuckyResult<NONPostObjectInputResponse> {
            error!("[{}] error: {}", ctx1.route, e);
            Ok(failed(&e.to_string()))
        })?;

        Ok(RouterHandlerPostObjectResult {
            action: RouterHandlerAction::Response,
            request: None,
            response: Some(Ok(post_object_response)),
        })
    }
}
