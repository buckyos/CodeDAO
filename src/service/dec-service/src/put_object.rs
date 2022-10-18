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
    pub async fn user_init(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<NONPostObjectInputResponse> {
        let object = &param.request.object;
        let buf = &object.object_raw;
        let user_info = UserInfo::clone_from_slice(buf)?;
        let user_name = user_info.name().to_string();
        info!("service revice user, user_name {}", user_name);
        let owner_id = user_info.owner();

        let map_user_key = format!("{}/{}", USER_LIST_PATH, &owner_id);
        let map_user_name_key = format!("{}/{}", USER_NAME_LIST_PATH, user_name);

        let env = self
            .stack
            .root_state_stub(None, Some(service_dec_id()))
            .create_path_op_env()
            .await?;

        if let Some(object_id) = env.get_by_path(map_user_name_key.clone()).await? {
            info!("user name in the name_map , check onwerid");
            let user = UserHelper::user(&self.stack, object_id).await?;
            if user.owner() != owner_id {
                let msg = format!("target user name [{}] already used", user_name);
                error!("{}", msg);
                return Ok(failed(&msg));
            }
        }

        // check key exist
        let result = env.get_by_path(map_user_key.clone()).await?;
        if result.is_some() {
            info!(
                "target user[{:?}] in the map,  use new name: {}",
                owner_id, user_name
            );
        }

        let object_id = user_info.desc().calculate_id();
        let _r = env
            .set_with_path(map_user_key, &object_id, None, true)
            .await?;
        let _r = env
            .set_with_path(map_user_name_key, &object_id, None, true)
            .await?;

        let root = env.commit().await;
        info!("add user name commit: {:?}", root);
        put_object(&self.stack, &user_info).await?;

        Ok(success(json!({"msg": "add user success in service"})))
    }

    pub async fn organization_new(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<NONPostObjectInputResponse> {
        let object = &param.request.object;
        let buf = &object.object_raw;
        let organization = Organization::clone_from_slice(buf)?;
        let name = organization.name();
        // 组名不可以和用户名冲突
        let map_user_name_key = format!("{}{}", USER_NAME_LIST_PATH, name);
        let env = self
            .stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        if env.get_by_path(map_user_name_key.clone()).await?.is_some() {
            let msg = format!("target name[{}] conflict with user name", name);
            error!("{}", msg);
            return Ok(failed(&msg));
        }

        let map_org_key = format!("{}/{}", ORG_LIST_PATH, name);
        if env.get_by_path(map_org_key.clone()).await?.is_some() {
            let msg = format!("target org name[{}] already used", name);
            error!("{}", msg);
            return Ok(failed(&msg));
        }

        let object_id = organization.desc().calculate_id();
        let r = env
            .set_with_path(&map_org_key, &object_id, None, true)
            .await?;
        info!("service add group in map {:?}  result:{:?}", map_org_key, r);
        let _ = env.commit().await;
        put_object(&self.stack, &organization).await?;
        info!("service add group success {}", organization.name());
        Ok(success(
            json!({"msg": "add organiztion success in service"}),
        ))
    }

    // repository_new
    // 新建 公开仓库
    pub async fn repository_new(
        &self,
        param: &RouterHandlerPostObjectRequest,
    ) -> BuckyResult<NONPostObjectInputResponse> {
        let object = &param.request.object;
        let buf = &object.object_raw;
        let repository = Repository::clone_from_slice(buf)?;

        // insert_repository(&self.stack, &repository).await?;
        let (repository_key, _) =
            RepositoryHelper::object_map_path(&repository.author_name(), &repository.name());
        let env = self
            .stack
            .root_state_stub(None, Some(service_dec_id()))
            .create_path_op_env()
            .await?;
        // check key exist
        info!("check for add public repository {}", repository_key);
        let result = env.get_by_path(&repository_key).await?;
        if result.is_some() {
            let msg = format!("repository[{}] was already created", &repository_key);
            error!("{}", msg);
            return Ok(failed(&msg));
        }

        put_object(&self.stack, &repository).await?;
        info!("put repository object local ok");

        let object_id = repository.desc().calculate_id();
        let r = env
            .set_with_path(&repository_key, &object_id, None, true)
            .await?;
        info!("service put repository{:?}  result:{:?}", repository_key, r);
        let _root = env.commit().await;
        info!(
            "service add public repository: {}/{}",
            repository.author_name(),
            repository.name()
        );
        Ok(success(json!({"msg": "add repository success in service"})))
    }
}
