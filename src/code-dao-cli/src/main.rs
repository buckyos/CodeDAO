use async_std::sync::Arc;
use clap::*;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;
//use git2::*;
use log::*;
mod post;
mod push;
use post::post_object;

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
    // do read, like git cat-file -p
}

fn init() {
    CyfsLoggerBuilder::new_app("code-dao-cli")
        .level("info")
        .console("info")
        .enable_bdt(Some("error"), Some("error"))
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
    info!("connect cyfs-runtime successfully");

    // get the current dir
    let cwd = std::env::current_dir().expect("failed to get codedao-cli cwd");
    // TOFIX
    let current_dir_name = cwd.into_iter().last();

    //let repo = git2::Repository::discover(cwd.clone()).expect("test");
    //info!("r {:?}", repo.workdir());

    //info!("current relative dir is {:?}", current_dir_name);
    //info!("current cwd is {:?}", cwd);

    let service = Service::new(Arc::clone(&stack));
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
    }
}

pub struct Service {
    stack: Arc<SharedCyfsStack>,
}

impl Service {
    pub fn new(stack: Arc<SharedCyfsStack>) -> Self {
        Self { stack }
    }

    // create repository
    pub async fn create(&self, name: &str) {
        let data = serde_json::json!({
            "name": name,
            "description":"aaa",
            "is_private":0,
            "author_type":"user",
        });
        let data = data.to_string();
        post_object(&self.stack, "repo/init", &data).await;
    }

    // push
    pub async fn push(&self) {
        main_test(self.stack.clone()).await.expect("push");
    }
}

async fn main_test(stack: Arc<SharedCyfsStack>) -> Result<(), git2::Error> {
    let name = "2022_1110";
    let test_dir_path = format!("/home/aa/test/{}", name);

    info!("current test git dir {}", test_dir_path);

    let ood = get_ood_device(&stack).await;
    //let owner = get_owner(&stack).await;
    let owner = owner(&stack);

    /// init stack util helper
    /// TODO  move put object to this
    let stack_util = Arc::new(StackUtil::new(
        Arc::clone(&stack),
        owner.clone(),
        ood.clone(),
    ));

    let name = format!("{}/{}", owner.to_string(), name);

    let branch = "main".to_string();
    let repo = git2::Repository::open(test_dir_path).expect("open repo failed");
    let push_helper = push::Push::new(Arc::new(repo), stack, stack_util, name, branch, ood, owner);
    //let index = push_helper.index()?;
    //info!("commit oid {}", index);

    push_helper.push().await.expect("check remote head");

    Ok(())
}
