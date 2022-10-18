#![windows_subsystem = "windows"]
use log::*;
use cyfs_git_base::*;

mod handler;
mod controller;
mod initor;
mod put_object;

use initor::DaohubServiceInit;


#[async_std::main]
async fn main() {
    DaohubServiceInit::init_process_check();
    DaohubServiceInit::init_logger();
    ConfigManager::new_oncecell_in_service();
    info!("get cyfs-git service dec id: {:?}", service_dec_id());

    let initor = DaohubServiceInit::new_with_stack(Some(service_dec_id())).await;
    initor
        .init_service_deviceid_check().await
        .init_stack_helper().await
        .init_sqlite_database().await
        .init_stack_handler().await;

    async_std::task::block_on(async_std::future::pending::<()>());
}
