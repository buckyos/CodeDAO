use crate::*;
use async_std::sync::Arc;
use async_trait::async_trait;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use cyfs_util::*;
use log::*;
use serde_json::json;

pub struct OnCommonPostHandle {
    pub stack: Arc<SharedCyfsStack>,
    pub put_object_helper: PutObjectHelper,
}

impl OnCommonPostHandle {
    pub fn new(stack: Arc<SharedCyfsStack>) -> Self {
        let put_object_helper = PutObjectHelper::new(Arc::clone(&stack));

        Self {
            stack,
            put_object_helper,
        }
    }
}

#[async_trait]
impl EventListenerAsyncRoutine<RouterHandlerPostObjectRequest, RouterHandlerPostObjectResult>
    for OnCommonPostHandle
{
    async fn call(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<RouterHandlerPostObjectResult> {
        let req_path = &param.request.common.req_path;
        info!("req path {:?}", req_path);

        if let Some(req_path) = req_path {
            let req_path = RequestGlobalStatePath::parse(req_path)?.req_path.unwrap();
            let mut req_path_info = req_path.trim_start_matches('/').split('/');
            let req_path_type = req_path_info.nth(1);
            info!("app handler: req_path type {:?}", req_path_type);

            if let Some("object") = req_path_type {
                let object_type = req_path_info.next().unwrap();
                info!(
                    "app handler: current req_path want to put object in,  type: {}",
                    object_type
                );
                let resp = match object_type {
                    "repository" => self.put_object_helper.repository_sync(param).await,
                    "organization" => self.put_object_helper.organization_sync(param).await,
                    _ => {
                        let msg = "req_path object type no match any object";
                        error!("{}", msg);
                        Ok(failed(msg))
                    }
                };
                if resp.is_err() {
                    error!(
                        "app req_path [{}] error: {:?}",
                        req_path,
                        resp.as_ref().err()
                    );
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

        // route match
        let post_object_response = match ctx.route.as_str() {
            "user/init" => user_init(ctx).await,
            "user/checkInit" => user_check_init(ctx).await,
            "user/setting" => user_setting(ctx).await,
            "user/list" => user_list(ctx).await,
            "user/getByName" => user_get_by_name(ctx).await,
            "user/info" => user_info(ctx).await,
            "organization/new" => organization_new(ctx).await,
            "organization/list" => organization_list(ctx).await,
            "organization/home" => organization_home(ctx).await,
            "organization/member" => organization_member(ctx).await,
            "organization/repository" => organization_repository(ctx).await,
            "organization/member/add" => organization_member_add(ctx).await,
            "repo/init" => repository_init(ctx).await,
            "repo/new" => repository_new(ctx).await,
            "repo/list" => repository_list(ctx).await,
            "repo/global/list" => repository_global_list(ctx).await,
            "repo/home" => repository_home(ctx).await,
            "repo/analysis/language" => repository_language_statistics(ctx).await,
            "repo/log/graph" => repository_log_graph(ctx).await,
            "repo/file" => repository_file(ctx).await,
            "repo/delete" => repository_delete(ctx).await,
            "remote/repo/delete" => remote_repository_delete(ctx).await,
            "repo/find" => repository_find(ctx).await,
            "repo/push/head" => repository_push_head(ctx).await,
            "repo/push" => repository_push(ctx).await,
            "repo/push/tag" => repository_push_tag(ctx).await,
            "repo/fetch/head" => repository_fetch_head(ctx).await,
            "repo/fetch" => repository_fetch(ctx).await,
            "repo/commits" => repository_commits(ctx).await,
            "repo/commit" => repository_commit(ctx).await,
            "repo/member" => repository_member_list(ctx).await,
            "repo/member/add" => repository_member_add(ctx).await,
            "repo/member/delete" => repository_member_delete(ctx).await,
            "repo/member/role" => repository_member_role(ctx).await,
            "repo/merge/compare" => repository_merge_compare(ctx).await,
            "repo/merge/create" => repository_merge_create(ctx).await,
            "repo/merges" => repository_merge_list(ctx).await,
            "repo/merge/detail" => repository_merge_detail(ctx).await,
            "repo/merge/accept" => repository_merge_accept(ctx).await,
            "repo/merge/compare/file" => repository_merge_compare_file(ctx).await,
            "repo/state/switch" => repository_setting_state_switch(ctx).await,
            "merge/list" => panel_merge_list(ctx).await,
            "issue/new" => issue_create(ctx).await,
            "issue/list" => panel_issue_list(ctx).await,
            "repo/issues" => repository_issue_list(ctx).await,
            "repo/issue" => repository_issue_detail(ctx).await,
            "repo/issue/comment" => repository_issue_comment(ctx).await,
            "repo/issue/close" => repository_issue_close(ctx).await,
            "repo/star" => repository_do_star(ctx).await,
            "authors" => authors(ctx).await,
            "friends" => friends(ctx).await,
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
