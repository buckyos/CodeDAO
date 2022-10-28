use cyfs_base::*;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;
// use cyfs_core::*;
use async_std::sync::Arc;
use log::*;
// use std::str::FromStr;
use crate::handler::*;
use crate::put_object::*;

pub struct DaohubServiceInit {
    pub stack: Arc<SharedCyfsStack>,
}

impl DaohubServiceInit {
    pub async fn new_with_stack(dec_id: Option<ObjectId>) -> Self {
        let stack = Arc::new(SharedCyfsStack::open_default(dec_id).await.unwrap());
        stack.wait_online(None).await.unwrap();
        Self { stack }
    }

    #[allow(dead_code)]
    pub async fn new_with_stack_simulator(
        service_http_port: &str,
        ws_port: &str,
        dec_id: Option<ObjectId>,
    ) -> Self {
        let parm_obj =
            SharedCyfsStackParam::new_with_ws_event(dec_id, service_http_port, ws_port).unwrap();
        let stack = Arc::new(SharedCyfsStack::open(parm_obj).await.unwrap());
        stack.wait_online(None).await.unwrap();
        Self { stack }
    }

    pub fn init_process_check() {
        let status = cyfs_util::process::check_cmd_and_exec(SQUARE_SERVICE_NAME);
        if status == cyfs_util::process::ProcessAction::Install {
            std::process::exit(0);
        }
    }

    pub fn init_logger() {
        CyfsLoggerBuilder::new_app(SQUARE_SERVICE_NAME)
            .level("info")
            .console("info")
            .enable_bdt(Some("off"), Some("off"))
            // .module("non-lib", Some("error"), Some("error"))
            .module("cyfs_lib", Some("error"), Some("error"))
            .module("sqlx", Some("error"), Some("error"))
            .build()
            .unwrap()
            .start();
    }

    pub async fn init_service_deviceid_check(self) -> Self {
        // mark global var
        is_ood_service(&self.stack).await;

        info!(
            "current ood is service: {:?}",
            OOD_IS_SERVICE.get().unwrap()
        );

        self
    }

    pub async fn init_stack_helper(self) -> Self {
        let owner = get_owner(&self.stack).await;

        // init STACK_ACTION
        let _ = STACK_ACTION.set(StackActionStruct {
            stack: self.stack.clone(),
            owner,
            dec_id: dec_id(),
        });
        info!("init stack action(helper) ok");
        self
    }

    // 初始化 db和table
    pub async fn init_sqlite_database(self) -> Self {
        CyfsGitDatabase::init().await.unwrap();
        let db = CyfsGitDatabase::instance().await.unwrap();
        db.init_tables_service().await.unwrap();

        // sync user 更改触发的时机
        // let _ = sync_users(&self.stack, db).await;

        self
    }

    pub async fn init_stack_handler(self) -> Self {
        let listener = OnCommonPostHandle {
            stack: self.stack.clone(),
            put_object_helper: PutObjectHelper::new(self.stack.clone()),
        };

        let meta = self.stack.root_state_meta_stub(None, Some(dec_id()));
        let access = AccessString::full();
        let _ = meta
            .add_access(GlobalStatePathAccessItem {
                path: DEC_SERVICE_HANDLER.to_string(),
                access: GlobalStatePathGroupAccess::Default(access.value()),
            })
            .await
            .unwrap();

        self.stack
            .router_handlers()
            .add_handler(
                RouterHandlerChain::Handler,
                "cyfs-git-service-post-object-handle",
                -1,
                None,
                Some(DEC_SERVICE_HANDLER.to_string()),
                RouterHandlerAction::Reject,
                Some(Box::new(listener)),
            )
            .unwrap();
        info!("init stack handlers ok");

        self
    }
}
