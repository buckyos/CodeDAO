use async_std::sync::Arc;
use cyfs_lib::*;
use std::env;
use std::path::PathBuf;
use cyfs_git_base::*;
use serde_json::{json};

mod cmd_handler;
mod config;
mod git_post;
mod runtime_launcher;
use cmd_handler::*;
use config::*;
use git_post::GitPost;

use crate::runtime_launcher::RuntimeLauncher;

#[async_std::main]
async fn main() {
    let c = Config::new();
    c.init();
    c.read();

    let git_version = option_env!("GIT_COMMIT_HAHS").unwrap_or_default(); // .expect("get GIT_COMMIT_HAHS failed");
    let build_date = option_env!("BUILDDATE").unwrap_or_default();
    let build_number = option_env!("VERSION").unwrap_or_default();
    let version = format!("1.0.{}.{}(built in {})", build_number, git_version, build_date);
    eprintln!("git-remote-cyfs version {}", version);
    eprintln!("current channel {}", ConfigManager::channel());
    
    
    let mut args = env::args();
    eprintln!("git-remote-cyfs argument size {:?} ", args.len());
    // check args number
    if args.len() == 1 {
        eprintln!("git-remote-cyfs quit, for no enough arguments");
        std::process::exit(0);
    }
    args.next();
    
    
    let repo_remote_name = args.next().expect("must provide alias");
    // 更改配置文件
    // --channel <[devtest,nightly]>
    if repo_remote_name == "--channel" {
        let channel_value = args.next().expect("channel value");
        c.switch_channel(channel_value);
        std::process::exit(0);
    }

    eprintln!("git target remote name {:?}", repo_remote_name);

    let url = args.next().expect("must provide url");
    // check protocal

    let git_dir = PathBuf::from(env::var("GIT_DIR").expect("GIT_DIR not set"));
    let current_dir = env::current_dir().expect("could not get pwd");
    eprintln!("git_dir {:?}", git_dir);
    eprintln!("current_dir {:?}", current_dir);

    let (space, repo_name) = {
        let space_and_name = url.split("//").into_iter().last().unwrap().to_string();
        // let result = url.as_str().replace(&format!("{}://", protocol), "");
        let mut result = space_and_name.split("/").into_iter();
        (
            result.next().expect("space").to_string(),
            result.next().expect("repo_name").to_string(),
        )
    };
    eprintln!("repository infomation {:?} {}", space, repo_name);

    RuntimeLauncher::launch().await;

    let stack = Arc::new(SharedCyfsStack::open_runtime(Some(dec_id())).await.unwrap());
    let result = stack.wait_online(Some(std::time::Duration::from_secs(5))).await;
    if result.is_err() {
        eprintln!("client connect runtime failed: {:?}", result.err().unwrap());
        std::process::exit(1);
    }
    eprintln!("cyfs open runtime");


    let git_post = Arc::new(GitPost::new(Arc::clone(&stack)).await);
    let resp = git_post
        .post(
            "repo/find",
            json!({"author_name": space, "name": repo_name}),
        )
        .await
        .unwrap();
    if resp.err == true {
        eprintln!("repo find result failed: {}", resp.msg.unwrap());
        std::process::exit(0);
    }

    let repository = resp
        .data
        .unwrap()
        .get("repository")
        .map(|x| x.to_owned())
        .unwrap();

    let mut cmd = GitHeplerCommand::new(
        Arc::clone(&stack),
        git_dir.clone(),
        current_dir.clone(),
        Arc::clone(&git_post),
        repository,
    );
    cmd.cmd_loop_dispatch().await;
    eprintln!("fin");

    // println!("path: ");

    // Ok(())
}
