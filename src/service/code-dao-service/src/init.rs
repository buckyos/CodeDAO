use cyfs_base::*;
use cyfs_lib::*;
// use cyfs_core::*;
use async_std::sync::Arc;

use crate::handler::*;
use cyfs_git_base::*;
use log::*;

pub struct DaohubInit {
    pub stack: Arc<SharedCyfsStack>,
}

impl DaohubInit {
    pub fn new(stack: Arc<SharedCyfsStack>) -> Self {
        DaohubInit { stack }
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

    #[allow(dead_code)]
    pub async fn init_current_space(self) -> Self {
        if set_current_space(&self.stack).await.is_err() {
            info!("set current space failed, maybe no init");
        }
        info!("init current space ok");
        self
    }

    pub async fn init_cache(self) -> Self {
        // 初始化 cache
        CyfsGitCache::new().unwrap();
        info!("init lru cache instance");

        self
    }

    pub async fn init_sqlite_database(self) -> Self {
        // 初始化 sqlite db和table
        CyfsGitDatabase::init().await.unwrap();
        let db = CyfsGitDatabase::instance().await.unwrap();
        db.init_tables_app().await.unwrap();
        info!("init sqlite ok");
        // sync repository
        // let _ = sync_repository_data(&self.stack, db).await;
        self
    }
    pub async fn init_stack_handler(self) -> BuckyResult<Self> {
        let listener = OnCommonPostHandle::new(self.stack.clone());
        let meta = self.stack.root_state_meta_stub(None, Some(dec_id()));
        let access = AccessString::full();
        meta.add_access(GlobalStatePathAccessItem {
            path: DEC_APP_HANDLER.to_string(),
            access: GlobalStatePathGroupAccess::Default(access.value()),
        })
        .await?;
        info!("add path access ok");
        //meta.add_access(GlobalStatePathAccessItem {
        //    path: DEC_APP_OBJECT.to_string(),
        //    access: GlobalStatePathGroupAccess::Default(access.value()),
        //})
        //.await?;

        self.stack.router_handlers().add_handler(
            RouterHandlerChain::Handler,
            "common-post-object-handler",
            -1,
            None,
            Some(DEC_APP_HANDLER.to_string()),
            RouterHandlerAction::Pass,
            Some(Box::new(listener)),
        )?;
        info!("init stack handlers ok");

        Ok(self)
    }
}
