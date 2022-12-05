use async_std::sync::Arc;
use clap::*;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;
//use git2::*;
use log::*;

mod post;
mod push;
mod service;

use post::post_object;
use service::Service;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Action,
    cmd: Option<String>,
}

#[derive(Subcommand)]
enum Action {
    Init { name: Option<String> },
    Push,
    Debug,
    // do read, like git cat-file -p
}

fn init() {
    CyfsLoggerBuilder::new_app("code-dao-cli")
        .level("info")
        .console("info")
        .enable_bdt(Some("error"), Some("error"))
        .disable_module(vec!["cyfs_lib"], cyfs_debug::LogLevel::Warn)
        .build()
        .unwrap()
        .start();

    ConfigManager::new_oncecell_with_content(
        r#"
    [main]
    channel="dev-test"
    deploy_owner_id="5r4MYfFQz9iEzjwHUpc79CwrvXqsh7xUzynkiTUEckxB"
    public_service_ood="5aSixgM1oBicrsUdS3nyKM1MA9AgiMEE2y2qFQ3jTTYB""#,
    );
}

fn main() {
    cyfs_debug::ProcessDeadHelper::patch_task_min_thread();
    async_std::task::block_on(main_run());
}

async fn main_run() {
    init();
    info!("init config successfully");
    //let stack = Arc::new(SharedCyfsStack::open_runtime(Some(dec_id())).await.unwrap());
    let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
    if let Err(err) = stack
        .wait_online(Some(std::time::Duration::from_secs(5)))
        .await
    {
        eprintln!("CodeDAO cli connect failed: {:?}", err);
        std::process::exit(1);
    }
    // info!("connect cyfs-runtime successfully");

    // get the current dir
    let cwd = std::env::current_dir().expect("failed to get codedao-cli cwd");
    // TOFIX
    let current_dir_name = cwd.into_iter().last();

    //let repo = git2::Repository::discover(cwd.clone()).expect("test");
    //info!("r {:?}", repo.workdir());

    //info!("current relative dir is {:?}", current_dir_name);
    //info!("current cwd is {:?}", cwd);

    let service = Service::new(Arc::clone(&stack)).await;
    let cli = Cli::parse();
    match &cli.command {
        Action::Init { name } => {
            info!("init repo name: {:?}", name);
            if let Some(name) = name {
                service.create(name).await;
            } else {
                let name = current_dir_name.unwrap().to_str().unwrap();
                info!("would use current dir name {}", name);
                service.create(name).await;
            }
        }
        Action::Push => {
            info!("cli action: push");
            service.push().await;
        }
        Action::Debug => {
            service.debug().await;
        }
    }
}
