use async_std::sync::Arc;
use clap::*;
use cyfs_debug::*;
use cyfs_git_base::{dec_id, owner, ConfigManager};
use cyfs_lib::*;
use git2::*;
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

    main_test(stack).await;
    std::process::exit(0);

    // get the current dir
    let cwd = std::env::current_dir().expect("failed to get codedao-cli cwd");
    let current_dir_name = cwd.into_iter().last();
    info!("current relative dir is {:?}", current_dir_name);
    info!("current cwd is {:?}", cwd);
    let git_dir = cwd.join(".git");

    // if git clone ,no need to this
    if !git_dir.exists() {
        error!("local repository's .git dir empty");
        std::process::exit(0);
    }

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
            info!("push push");
            service.push(git_dir).await;
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
    pub async fn push(&self, git_dir: std::path::PathBuf) {
        // read the .git/objects
        let head_file = git_dir.join("HEAD");
        let ref_head = std::fs::read_to_string(head_file).expect("read HEAD failed");
        info!("current branch: {}", ref_head);

        if let Some(branch) = ref_head.split("/").last() {
            let branch = branch.trim().to_string();

            let ref_head = ref_head.clone();
            let (_, head_file_path) = ref_head.rsplit_once(":").unwrap();

            let head_file_path = head_file_path.trim();
            let ref_file = git_dir.join(head_file_path);

            info!("ref file: {:?}", ref_file.display());
            // get commit HEAD oid
            let ref_result = std::fs::read_to_string(ref_file);
            if ref_result.is_err() {
                println!("no commit yet");
            }

            // git status
            let index_file = git_dir.join("index");
            if index_file.is_file() {
                println!("git stash no clean; need to commit?");
            }

            let head_oid = ref_result.unwrap().trim().to_string();
            info!("{}: {}", branch, head_oid);

            //
            // read commit oid
            // TODO commit oid check rootstate

            // commit object path

            let commit_object_path = git_dir
                .join("objects")
                .join(&head_oid[..2])
                .join(&head_oid[2..]);
            info!("path: {}", commit_object_path.display());

            let commit_message = std::fs::read(commit_object_path).expect("read commit failed");
            info!("why {:?}", commit_message);
        }
    }
}

async fn main_test(stack: Arc<SharedCyfsStack>) -> Result<(), git2::Error> {
    let test_dir_path = "/home/aa/test/2022_1110";

    let people = owner(&stack);
    let name = format!("{}/{}", people.to_string(), "2022_1110");

    let repo = Repository::open(test_dir_path).expect("open repo failed");
    let push_helper = push::Push::new(Arc::new(repo), stack, name);
    let index = push.index()?;
    info!("commit oid {}", index);

    push_helper.push().await.expect("check remote head");

    Ok(())
}
