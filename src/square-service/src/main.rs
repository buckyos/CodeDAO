// #![windows_subsystem = "windows"]
use async_std::task::block_on;
use cyfs_debug::ProcessDeadHelper;
use cyfs_git_base::*;
use log::*;
mod controller;
mod handler;
mod initor;
mod put_object;

use initor::DaohubServiceInit;

fn main() {
    ProcessDeadHelper::patch_task_min_thread();
    block_on(main_run());
}

async fn main_run() {
    DaohubServiceInit::init_process_check();
    DaohubServiceInit::init_logger();
    ConfigManager::new_oncecell_in_service();
    info!("CodeDAO square service dec id: {:?}", service_dec_id());
    let initor = DaohubServiceInit::new_with_stack(Some(service_dec_id())).await;
    initor.start().await.unwrap();
    // Simulator debugging
    // let initor = DaohubServiceInit::new_with_stack_simulator(
    //     "http://127.0.0.1:21000",
    //     "ws://127.0.0.1:21001",
    //     Some(service_dec_id()),
    // )
    // .await;
    block_on(async_std::future::pending::<()>());
}
