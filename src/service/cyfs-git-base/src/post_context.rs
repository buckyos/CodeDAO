use cyfs_lib::*;
use cyfs_base::*;
use async_std::sync::Arc;
use log::*;
use crate::*;


pub struct PostContext{
    pub stack: Arc<SharedCyfsStack>,
    pub data: String,
    pub route: String, 
    pub id: String,
    pub caller: ObjectId,
    pub source_device: DeviceId,
}

impl PostContext {
    pub fn new_by_param(param: &RouterHandlerPostObjectRequest, stack: Arc<SharedCyfsStack>) -> Arc<Self> {
        let (route, value, caller, object_id) = decode_text(&param).unwrap();
	let source = param.request.common.source.zone.device.as_ref().unwrap();
        Arc::new(Self{
            stack,
            data: value,
            route,
            id: object_id,
            caller,
            source_device: source.clone(),
        })
    }

    pub async fn stack_env(&self) -> BuckyResult<PathOpEnvStub>{
        let env = self.stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
        Ok(env)
    }

    pub async fn stack_single_env(&self) -> BuckyResult<SingleOpEnvStub>{
        let env = self.stack.root_state_stub(None, Some(dec_id())).create_single_op_env().await?;
        Ok(env)
    }

    pub fn is_other_caller(&self) -> bool {
        let is_other = !self.caller.eq(&owner(&self.stack));
        is_other
    }

    
    /// check_space_proxy_request
    /// 如果是local 返回 None
    /// check space is local, if not, request remote ood
    pub async fn check_space_proxy_request(&self, space: &str) -> BuckyResult<Option<NONPostObjectInputResponse>> {

        // TODO
        if is_current_space(space)? {
            info!("target space({}) hit the current space cache", space);
            return Ok(None)
        }

        if check_space_local(&self.stack, space).await? {
            return Ok(None)
        }

        let result = request_other_ood(&self.stack, space, &self.route, &self.data).await?;
        Ok(Some(result))
    }

    pub fn  repository_helper(&self, author_name: String, name: String) -> RepositoryHelper {
        RepositoryHelper::new(self.stack.clone(), author_name, name)
    }
}
