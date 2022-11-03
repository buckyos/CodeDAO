
use cyfs_base::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use log::*;
use crate::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GitCommit {
    pub object_id: String,
    pub parent: String,
    pub parent2: String,
    pub tree: String,
    pub payload: String,
    pub author: GitCommitAuthor,
    pub committer: GitCommitAuthor,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GitCommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

impl GitCommitAuthor {
    pub fn parse(source: impl Into<String>) -> GitCommitAuthor {
        // 这里不能基于空格去 splite, 因为名字可能带有空格
        let source = source.into();
        let re = regex::Regex::new(r"(?P<name>.+)[ ]<(?P<email>.+)?>[ ](?P<date>.+)").unwrap();
        let caps = re.captures(&source);
        let result = caps.unwrap();
        let name = result.name("name").unwrap().as_str().to_string();
        let email = result.name("email").unwrap().as_str().to_string();
        let date = result.name("date").unwrap().as_str().to_string();

        GitCommitAuthor{
            name,
            email,
            date,
        }
    }
}


/// git_read_commit_object
/// 根据object_id (从show ref获取这个oid) 去查询commit信息
pub fn git_read_commit_object(repo_dir: PathBuf, object_id: &str) -> BuckyResult<GitCommit> {
    let stdout = git_exec_base(repo_dir, ["cat-file", "-p", object_id]);
    if stdout.is_ok() {
        let stdout = stdout.unwrap();
        let mut lines = stdout.lines();
        let mut commit = GitCommit{
            object_id: object_id.to_string(), 
            ..Default::default()  
        };
        loop {
            let line = lines.next().unwrap();
            if line == ""  { // 空行退出，空行后面的是 commit message
                break;
            }
            if line.starts_with("tree") {
                commit.tree = line.replace("tree ", "").to_string();
            } else if line.starts_with("parent")  { // parent 可能有两个（通过merge生成的commit
                if commit.parent == "" {
                    commit.parent = line.replace("parent ", "").to_string();
                } else {
                    commit.parent2 = line.replace("parent ", "").to_string();
                }
            } else if line.starts_with("author")  {
                let author = line.replace("author ", "").to_string();
                commit.author =  GitCommitAuthor::parse(author);
            } else if line.starts_with("committer")  {
                let committer = line.replace("committer ", "").to_string();
                commit.committer =  GitCommitAuthor::parse(committer);
            }
        }
            // tree f2fc5824de4d19dcea769ee6961efc48be139999
            // parent f6bb053892d861682de81d9cd23d84bd07db1d79
            // parent 0ac9b995f19bb3e2f3d9fbfbf6428db11313f119
            // author sunxinle <alexsunxl@163.com> 1647428218 +0800
            // committer sunxinle <alexsunxl@163.com> 1647428218 +0800
            // test commit messge
        // git commit的message 合并回来
        let payload = lines.collect::<Vec<_>>().join('\n'.to_string().as_str());
        commit.payload = payload;
        debug!("git git_read_commit_object {:?}", commit);
        return Ok(commit)
    }
    Err(BuckyError::new(BuckyErrorCode::Failed, "git read commit object error"))
}


#[cfg(test)]
mod main_tests {
    use super::*;
    #[async_std::test]
    async fn test_git_read_commit_object() {
        let repo_dir= PathBuf::from(r#"E:\app\cyfs-git-rust\"#);
        let result = git_read_commit_object(repo_dir, "d89e4038f56b8d26d2d12d3f9dc9c5bb4eac03d3").unwrap();
        println!("{:#?}", result);

    }
    // 通过合并创建出来的commit
    #[async_std::test]
    async fn test_git_read_commit_object_two_parents() {
        let repo_dir= PathBuf::from(r#"D:\app\aaa"#);
        let result = git_read_commit_object(repo_dir, "fb7c392f05a3e02373ba14846e4caa9ddf44eb8d").unwrap();
        println!("{:#?}", result);
    }

    #[async_std::test]
    async fn test_git_read_commit_parse() {
        let message = "Cobin Bluth <cbluth@gmail.com> 1575290569 +0100";
        let re = regex::Regex::new(r"(?P<name>.+)[ ]<(?P<email>.+)?>[ ](?P<date>.+)").unwrap();
        // let re = regex::Regex::new(r"(?P<name>.+)[ ]<").unwrap();
        let caps = re.captures(message);
        let result = caps.unwrap();
        println!("{:#?}", result);
        let name = result.name("name").unwrap().as_str().to_string();
        let email = result.name("email").unwrap().as_str().to_string();
        let date = result.name("date").unwrap().as_str().to_string();
        println!("{:#?}", name);
        println!("{:#?}", email);
        println!("{:#?}", date);
    }
}