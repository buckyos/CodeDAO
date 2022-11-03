//#![windows_subsystem = "windows"]
use async_std::sync::Arc;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use std::str::FromStr;

mod controller;
mod handler;
mod service;
mod put_object;

use controller::*;
use put_object::*;
use service::CodeDaoService;


fn main() {
    cyfs_debug::ProcessDeadHelper::patch_task_min_thread();
    async_std::task::block_on(main_run());
}

async fn main_run() {
    let status = cyfs_util::process::check_cmd_and_exec(CYFS_GIT_DEC_APP_NAME);
    if status == cyfs_util::process::ProcessAction::Install {
        std::process::exit(0);
    }
    ConfigManager::new_oncecell();

    CyfsLoggerBuilder::new_app(CYFS_GIT_DEC_APP_NAME)
        .level("info")
        .console("info")
        .enable_bdt(Some("off"), Some("off"))
        .module("cyfs_lib", Some("error"), Some("error"))
        .module("sqlx", Some("error"), Some("error"))
        .build()
        .unwrap()
        .start();

    info!("CodeDAO decID: {:?}", dec_id());
    let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
    // Simulator debugging
    // let parm_obj = SharedCyfsStackParam::new_with_ws_event(
    //     Some(dec_id()),
    //     "http://127.0.0.1:21000",
    //     "ws://127.0.0.1:21001",
    // )
    // .unwrap();
    // let stack = Arc::new(SharedCyfsStack::open(parm_obj).await.unwrap());
    stack.wait_online(None).await.unwrap();

    let service = CodeDaoService::new(stack.clone());
    service.start().await.unwrap();

    async_std::task::block_on(async_std::future::pending::<()>());
}
