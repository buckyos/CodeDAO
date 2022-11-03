use cyfs_base::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use log::*;
use crate::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct GitLogGraphLine {
    pub hash: String,
    pub parents: Vec<String>,
    pub head: Vec<String>,
    pub message: String,
    pub timestamp: String,
}

impl GitLogGraphLine {
    pub fn parse(content: &str) -> GitLogGraphLine {
        let mut result = content.split("|");
        let hash = result.next().unwrap().to_string();
        let parents = result.next().unwrap().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let timestamp = result.next().unwrap().to_string();
        let head = result.next().unwrap()
            .replace("HEAD ->", "")
            .trim().trim_matches(|c| c == '(' || c == ')')
            .split(", ").filter_map(|x| {
                if x == "" {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect::<Vec<String>>();
        let message = result.next().unwrap().to_string();
        
        // let parents =  result.nth(1).unwrap().to_string().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        GitLogGraphLine {
            hash,
            parents,
            timestamp,
            head,
            message,
        }
    }
}

/// git_log_graph
/// 文件内容
pub fn git_log_graph(repo_dir: PathBuf, offset: i32, limit: i32) -> BuckyResult<Vec<GitLogGraphLine>> {
    // git log --all --date-order --pretty="%h|%p|%at|%d|%s" -n 20 --skip=40
    let skip = format!("--skip={}", offset);
    let stdout = git_exec_base(repo_dir.clone(), ["log", "--all", "--date-order", "--pretty=%h|%p|%at|%d|%s", "-n", &limit.to_string(), &skip]);
    let output = stdout.unwrap();
    let graphs = output.lines().map(|line| { 
        GitLogGraphLine::parse(line)
    }).collect::<Vec<GitLogGraphLine>>();
    info!("git log output: {:?}", graphs);
    Ok(graphs)
}

// test
#[cfg(test)]
mod main_tests {
    use super::*;
    use cyfs_debug::*;

    #[async_std::test]
    async fn test_git_log_graph() {
        CyfsLoggerBuilder::new_app(CYFS_GIT_DEC_APP_NAME)
            .level("info")
            .console("info")
            .enable_bdt(Some("off"), Some("off"))
            .build()
            .unwrap()
            .start();
        let repo_dir= PathBuf::from(r#"E:\app\cyfs-git-rust"#);
        let graph = git_log_graph(repo_dir, 0, 20);
        info!("graph {:?}", graph);
    }

}