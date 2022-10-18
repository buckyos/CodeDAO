use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde_json::json;

pub struct PutObjectHelper {
    pub stack: Arc<SharedCyfsStack>,
}

impl PutObjectHelper {
    pub fn new(stack: Arc<SharedCyfsStack>) -> Self {
        Self { stack }
    }

    // repository_sync
    pub async fn repository_sync(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<NONPostObjectInputResponse> {
        let object = &param.request.object;
        let buf = &object.object_raw;
        let repository = Repository::clone_from_slice(buf)?;

        let (repository_key, _) =
            RepositoryHelper::object_map_path(&repository.author_name(), &repository.name());
        let env = self
            .stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        // check key exist
        info!("check for add public repository {}", repository_key);
        let result = env.get_by_path(&repository_key).await?;
        if result.is_some() {
            let msg = format!("repository[{}] was already created", &repository_key);
            error!("{}", msg);
            return Ok(failed(&msg))
        }


        let object_id = repository.desc().calculate_id();
        let r = env
            .set_with_path(&repository_key, &object_id, None, true)
            .await?;
        info!("service put repository{:?}  result:{:?}", repository_key, r);
        let _root = env.commit().await;
        let r = put_object(&self.stack, &repository).await?;
        info!(
            "sync repository: {}/{} success",
            repository.author_name(),
            repository.name()
        );
        Ok(success(json!({"msg": "sync repository success"})))
    }


    pub async fn organization_sync(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<NONPostObjectInputResponse> {
        let object = &param.request.object;
        let buf = &object.object_raw;
        let organization = Organization::clone_from_slice(buf)?;
        let map_org_key  = format!("{}/{}" ,ORG_LIST_PATH, organization.name());
        let env = self
            .stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        // check key exist
        info!("check for add public organization key {}", map_org_key);
        let result = env.get_by_path(&map_org_key).await?;
        if result.is_some() {
            let msg = format!("organization[{}] was already created", &map_org_key);
            error!("{}", msg);
            return Ok(failed(&msg))
        }


        let object_id = organization.desc().calculate_id();
        let r = env
            .set_with_path(&map_org_key, &object_id, None, true)
            .await?;
        info!("service put organization{:?}  result:{:?}", map_org_key, r);
        let _root = env.commit().await;
        let r = put_object(&self.stack, &organization).await?;
        info!(
            "sync organization: {} success",
            organization.name()
        );
        Ok(success(json!({"msg": "sync organization success"})))
    }

}
