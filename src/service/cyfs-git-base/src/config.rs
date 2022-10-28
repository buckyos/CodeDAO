use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_core::*;
use cyfs_lib::*;
use log::*;
use once_cell::sync::OnceCell;
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ConfigRoot {
    pub main: ConfigMain,
}

#[derive(Debug, Deserialize)]
pub struct ConfigMain {
    pub deploy_owner_id: String,
    pub public_service_ood: String,
    pub channel: String,
}

pub struct ConfigManager {
    app_config: ConfigMain,
    app_dec_id: ObjectId,
    app_service_dec_id: ObjectId,
}
pub static CONFIG_MANAGER_INSTANCE: OnceCell<ConfigManager> = OnceCell::new();
impl ConfigManager {
    /// new
    /// read config file
    pub fn new_with_content(content: &str) -> Self {
        let root: ConfigRoot = toml::from_str(content).unwrap();
        let owner = root.main.deploy_owner_id.as_str();
        let owner = ObjectId::from_str(owner).unwrap();
        let app_dec_id = DecApp::generate_id(owner.clone(), CODE_DAO_SERVICE_NAME);
        let app_service_dec_id = DecApp::generate_id(owner, SQUARE_SERVICE_NAME);

        Self {
            app_config: root.main,
            app_dec_id,
            app_service_dec_id,
        }
    }

    pub fn new_oncecell() {
        let content =
            std::fs::read_to_string("config/config.toml").expect("config config.toml file no ok");
        let root: ConfigRoot = toml::from_str(&content).unwrap();

        let owner = root.main.deploy_owner_id.as_str();
        let owner = ObjectId::from_str(owner).unwrap();
        let app_dec_id = DecApp::generate_id(owner.clone(), CODE_DAO_SERVICE_NAME);
        let app_service_dec_id = DecApp::generate_id(owner, SQUARE_SERVICE_NAME);
        let config_manager = Self {
            app_config: root.main,
            app_dec_id,
            app_service_dec_id,
        };
        let _ = CONFIG_MANAGER_INSTANCE.set(config_manager);
        info!("dec app set config manager global instance");
    }

    pub fn new_oncecell_in_service() {
        let content =
            std::fs::read_to_string("config/config.toml").expect("config config.toml file no ok");
        let root: ConfigRoot = toml::from_str(&content).unwrap();

        let owner = root.main.deploy_owner_id.as_str();
        let owner = ObjectId::from_str(owner).unwrap();
        // let app_dec_id = DecApp::generate_id(owner.clone(), CODE_DAO_SERVICE_NAME);
        let app_service_dec_id = DecApp::generate_id(owner, SQUARE_SERVICE_NAME);
        let app_dec_id = app_service_dec_id.clone();

        let config_manager = Self {
            app_config: root.main,
            app_dec_id,
            app_service_dec_id,
        };
        let _ = CONFIG_MANAGER_INSTANCE.set(config_manager);
        info!("service set config manager global instance OK");
    }

    pub fn new_oncecell_with_content(content: &str) {
        let config_manager = ConfigManager::new_with_content(content);
        let _ = CONFIG_MANAGER_INSTANCE.set(config_manager);
        println!("set config manager global instance");
    }

    pub fn get_service_ood_device() -> ObjectId {
        let config = CONFIG_MANAGER_INSTANCE.get().unwrap();
        let ood_id = config.app_config.public_service_ood.clone();
        ObjectId::from_str(&ood_id).unwrap()
    }

    pub fn get_app_dec_id() -> ObjectId {
        let config = CONFIG_MANAGER_INSTANCE.get().unwrap();
        config.app_dec_id
    }

    pub fn get_app_service_dec_id() -> ObjectId {
        let config = CONFIG_MANAGER_INSTANCE.get().unwrap();
        config.app_service_dec_id
    }

    pub fn channel() -> String {
        let config = CONFIG_MANAGER_INSTANCE.get().unwrap();
        config.app_config.channel.clone()
    }
}

/// dec_id
/// dec_app_id 或者dec_service_id
/// in service is service_dec_id
pub fn dec_id() -> ObjectId {
    // *APP_DEC_ID
    ConfigManager::get_app_dec_id()
}

/// service_dec_id
/// 全局应用（广场）的dec id 只用来启动协议栈。object的创建还是按照dec_id来
pub fn service_dec_id() -> ObjectId {
    // *SERVICE_DEC_ID
    ConfigManager::get_app_service_dec_id()
}

/// service_ood_device
/// 全局应用（广场）的ood device_id
pub fn service_ood_device() -> ObjectId {
    // *SERVICE_DEC_ID
    ConfigManager::get_service_ood_device()
}

pub static OOD_IS_SERVICE: OnceCell<bool> = OnceCell::new();
pub static CURRET_SPACE: OnceCell<String> = OnceCell::new();

pub async fn set_current_space(stack: &Arc<SharedCyfsStack>) -> BuckyResult<()> {
    let user = UserHelper::get_current_user(stack).await?;
    CURRET_SPACE.set(user.name().to_string()).map_err(|e| {
        error!("set_current_space: {:?}", e);
        BuckyError::new(
            BuckyErrorCode::InternalError,
            format!("set_current_space error:{}", e),
        )
    })?;
    Ok(())
}

pub fn is_current_space(space: &str) -> BuckyResult<bool> {
    let value = CURRET_SPACE.get();
    if value.is_none() {
        return Ok(false);
    }
    let current_space = value.unwrap().to_string();
    Ok(current_space.eq(space))
}

// check is global service
// TODO
pub async fn is_ood_service(stack: &Arc<SharedCyfsStack>) {
    let info = stack
        .util_service()
        .get_device_static_info(UtilGetDeviceStaticInfoOutputRequest {
            common: UtilOutputRequestCommon {
                ..Default::default()
            },
        })
        .await
        .unwrap();

    // check
    let service_ood_device = service_ood_device();
    if info.info.is_ood_device
        && info.info.ood_device_id.to_string() == service_ood_device.to_string()
    {
        OOD_IS_SERVICE.set(true).unwrap();
    } else {
        OOD_IS_SERVICE.set(false).unwrap();
    }
}
