
use async_std::sync::Arc;
use cyfs_base::BuckyResult;
use cyfs_base::*;
use cyfs_lib::*;
use serde_json::Value;
use cyfs_git_base::*;


pub struct GitPost {
    pub stack: Arc<SharedCyfsStack>,
    pub ood_device: ObjectId,
}
impl GitPost {
    pub async fn new(stack: Arc<SharedCyfsStack>) -> Self {
        let device = get_ood_device(&stack).await;
        Self {
            stack,
            ood_device: device,
        }
    }
    pub async fn post(
        &self,
        route: &str,
        data: Value,
    ) -> BuckyResult<PostObjectCommonResponseData> {
        let req_path =  Some(format!("{}/api", DEC_APP_HANDLER));
        let (_, result) = post_target(&self.stack, self.ood_device, route, &data.to_string(), req_path)
            .await
            .unwrap();
        let response = PostObjectCommonResponseData::parse(&result).unwrap();
        Ok(response)
    }
}
