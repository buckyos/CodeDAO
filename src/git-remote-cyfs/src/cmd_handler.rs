use std::{path::PathBuf, sync::Mutex};
use async_std::sync::Arc;
use cyfs_lib::*;
use cyfs_base::*;
use cyfs_git_base::*;
use serde_json::{json, Value};
use std::io::{self, BufRead};
use tempfile::Builder;
use std::str::FromStr;
use crate::git_post::GitPost;


pub struct GitHeplerCommand {
    pub current_dir: PathBuf,
    pub git_dir: PathBuf,
    pub stack: Arc<SharedCyfsStack>,
    pub git_post: Arc<GitPost>,
    pub repository: Value,
    pub push_head_remote_ref_hash: Option<String>,
    fetch_branch_mark: Mutex<Vec<String>>,
    is_clone: bool,
}

impl GitHeplerCommand {
    pub fn new(stack: Arc<SharedCyfsStack>, git_dir: PathBuf, current_dir: PathBuf, git_post: Arc<GitPost>,repository: Value) -> Self{
        let is_clone = git_dir.is_absolute();
        Self{
            stack,
            git_dir,
            current_dir,
            git_post,
            repository,
            push_head_remote_ref_hash: None,
            fetch_branch_mark: Mutex::new(vec![]),
            is_clone: is_clone,
        }
    }
    pub async fn cmd_loop_dispatch(&mut self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Err(_) => {
                    eprintln!("stdin read lines error");
                    break;
                },
                Ok(cmd_line) => {
                    eprintln!(">> {}", cmd_line);
                    let mut iter = cmd_line.split_ascii_whitespace();
                    let cmd = iter.next();
                    let arg1 = iter.next();
                    let arg2 = iter.next();
            
                    match (cmd, arg1, arg2) {
                        (Some("push"), Some(ref_arg), None) => self.cmd_push(ref_arg).await,
                        (Some("fetch"), Some(sha), Some(name)) => self.cmd_fetch(sha, name).await,
                        (Some("capabilities"), None, None) => self.cmd_capabilities(),
                        (Some("list"), None, None) => self.cmd_list().await,
                        (Some("list"), Some("for-push"), None) => self.cmd_list_for_push().await,
                        (None, None, None) => self.cmd_complete(),
                        _ => self.cmd_unknown(),
                    }
                },
            }
        }
    }

    pub fn cmd_capabilities(&self) {
        println!("*push");
        println!("*fetch");
        println!();
    }

    pub async fn cmd_fetch(&self, remote_sha: &str, remote_ref: &str) {
        eprintln!("cmd fetch remote_sha {} branch_name {} ", remote_sha, remote_ref);
        let branch = remote_ref.clone().split("/").last().unwrap();

        let mut lock = self.fetch_branch_mark.lock().unwrap();
        if lock.contains(&remote_ref.to_string()) {
            eprintln!("cmd fetch branch[{}] 's data aleady fetch. pass this {} ", remote_ref, remote_sha);
            return
        }
        
        // get local branch's commit_oid
        let local_sha = if self.is_clone {
            "".to_string()
        } else {
            let result = git_show_target_branch_ref(self.current_dir.clone(), branch);
            if let Ok(hash) =  result {
                hash
            } else {
                eprintln!("remote track, local no branch: {:?} ", branch);
                "".to_string()
            }
        };


        let resp = self.git_post.post("repo/fetch", json!({
            "author_name": self.repository["author_name"].as_str().unwrap(), 
            "name": self.repository["name"].as_str().unwrap(),
            "branch": branch,
            "local_hash": local_sha,
            // "hash": remote_sha,
        })).await.unwrap();
        eprintln!("cmd fetch post result {:?} ", resp);
        if resp.err {
            eprintln!("fetch error {:?}", resp.msg.unwrap());
            std::process::exit(1);
        }
        let data = resp.data.unwrap();
        
        let file_id = ObjectId::from_str(data["file_id"].as_str().unwrap()).unwrap();
        let device_id = DeviceId::from_str(data["device_id"].as_str().unwrap()).unwrap();

        let tmp_dir = Builder::new()
            .prefix("cyfs_git_fetch")
            .tempdir();
        if tmp_dir.is_err() {
            eprintln!("tmp_dir error {:?}", tmp_dir.err());
            std::process::exit(1);
        }
        let tmp_dir = tmp_dir.unwrap();
        let tmp_dir = tmp_dir.path();
        let local_path = tmp_dir.join(&format!("pack.{}", file_id));
        eprintln!("cmd fetch crate file task {:?} ", file_id);

        // create task to runtime: download target file
        let ood_device = get_ood_device(&self.stack).await;
        if ood_device.to_string() == device_id.to_string() {
            let _ = file_task(&self.stack, file_id, local_path.clone(), device_id, None).await.unwrap();
        } else {
            eprintln!("download file from other's ood");
            let target_ood_device = ObjectId::try_from(device_id.clone()).unwrap();
            let _ = file_task(&self.stack, file_id, local_path.clone(), device_id, Some(target_ood_device)).await.unwrap();
        }

        eprintln!("cmd fetch pack file download ok: {:?} ", local_path);

        let bundle_file_path = local_path.to_str().unwrap();
        let pull_cwd = if self.is_clone {
            let cwd = self.git_dir.to_str().unwrap().replace(".git", "");
            let cwd = PathBuf::from_str(&cwd).unwrap();
            cwd
        } else {
            self.current_dir.clone()
        };
        

        git_unpack_objects(pull_cwd.clone(), PathBuf::from_str(bundle_file_path).unwrap());
        
        if self.is_clone {
            eprintln!("fetch update ref");
            git_update_ref(pull_cwd, branch, remote_sha);
        }

        lock.push(remote_ref.to_string());
        eprintln!("{} fetch ok", remote_ref.to_string());

        println!(); 
    }

    pub async fn cmd_push(&self, push_ref: &str) {
        let (_force, branch, local_sha, dst_ref)  = self._parse_ref(push_ref).unwrap();

        // TODO check if remote could be push

        let tmp_dir = Builder::new()
            .prefix("cyfs_git_push")
            .tempdir();
        if tmp_dir.is_err() {
            eprintln!("tmp_dir error {:?}", tmp_dir.err());
            std::process::exit(1);
        }
        let tmp_dir = tmp_dir.unwrap();
        let tmp_dir = tmp_dir.path();
        let target_bundle_file = tmp_dir.join(&format!("bundle.{}", branch));
        eprintln!("cmd push bundle file path {:?} ", target_bundle_file);



        let git_rev_list_arg = if self.push_head_remote_ref_hash.is_none() {
            branch.clone()
        } else {
            format!("{}..{}", self.push_head_remote_ref_hash.clone().unwrap(), branch)
        };
        eprintln!("cmd push bundle create git_rev_list_arg: {:?} ", git_rev_list_arg);

        let push_complete = move |resp: PostObjectCommonResponseData, dst_ref: String |  {
            // reponse message to std
            let message = if resp.err {
                format!("error {}", dst_ref)
            } else {
                format!("ok {}", dst_ref)
            };
            println!("{}", message);
            eprintln!("<< {}", message);
            println!();
        };

        // check: is only need to send the tag/head.
        let objects_count = git_rev_list_count(self.current_dir.clone(), &git_rev_list_arg).expect("rev-list the remote and current failed");
        if objects_count == 0 {
            eprintln!("only send tag or head to service");
            // only push tag
            let resp = self.git_post.post("repo/push/tag", json!({
                "author_name": self.repository["author_name"].as_str().unwrap(), 
                "name": self.repository["name"].as_str().unwrap(),
                "branch": branch,
                "ref_hash": local_sha,
            })).await.expect("push git bundle failed");
            push_complete(resp, dst_ref);
            return
        } 

        // git bundle
        if let Err(err) = git_bundle_create(
            self.current_dir.clone(), 
            target_bundle_file.to_str().unwrap(),
            &git_rev_list_arg) {
            eprintln!("git_bundle_create failed {:?}", err);
        }
        let resp = self._push_bundle(target_bundle_file, branch, local_sha).await.expect("push git bundle failed");
        push_complete(resp, dst_ref);

    }

    pub async fn _push_bundle(&self, target_bundle_file: PathBuf, branch: String, local_sha: String) -> BuckyResult<PostObjectCommonResponseData>  {
        let file_id = publish_file(&self.stack, target_bundle_file).await.unwrap();
        eprintln!("cmd push publish bundle file result {:?} ", file_id);
        let owner = get_owner(&self.stack).await;

        let resp = self.git_post.post("repo/push", json!({
            "author_name": self.repository["author_name"].as_str().unwrap(), 
            "name": self.repository["name"].as_str().unwrap(),
            "pack_file_id": file_id,  // 这个pack可以是 delta的
            "branch": branch,
            "ref_hash": local_sha,
            "dec_id": dec_id().to_string(),
            "runtime_device_id": get_local_device(&self.stack),
            "user_id": owner,
        })).await.unwrap();
        eprintln!("cmd push post result {:?} ", resp);

        let commit = git_read_commit_object(self.current_dir.clone(), &local_sha).unwrap();
        let tree_id = commit.tree;
        root_tree_object_loop_insert_map(
            &self.stack, 
            owner, 
            self.current_dir.clone(),
            &tree_id,
            self.repository["author_name"].as_str().unwrap(),
            self.repository["name"].as_str().unwrap(),
        ).await.unwrap();
        eprintln!("set tree object into objectmap all ok. root_tree_id {:?}", tree_id);
        Ok(resp)
    }

    pub async fn cmd_list(&self) {
        eprintln!("cmd_list start request repo/fetch/head result");
        let data = self.git_post.post(
            "repo/fetch/head", 
            json!({
                "author_name": self.repository["author_name"].as_str().unwrap(), 
                "name": self.repository["name"].as_str().unwrap(),
                "is_clone": self.is_clone,
            }),
        ).await.unwrap();
        
        if data.err {
            eprintln!("fetch head error {:?}", data.msg.unwrap());
            std::process::exit(1);
        }
        let remote_refs = data.data.unwrap()["refs"]
            .as_array()
            .map(|v| v.to_owned()).unwrap();

        if remote_refs.is_empty() {
            println!();
            return 
        }
        for remote_ref_value in remote_refs {
            let remote_ref_hash = remote_ref_value["hash"].as_str().unwrap();
            let remote_ref_name = remote_ref_value["ref_name"].as_str().unwrap();
            let message = format!("{} {}", remote_ref_hash, remote_ref_name);
            println!("{}", message);
            eprintln!("<< {}", message);
        }
        println!();
    }

    pub async fn cmd_list_for_push(&mut self) {
        eprintln!("cmd_list start request repo/push/head");
        let data = json!({
            "author_name": self.repository["author_name"].as_str().unwrap(), 
            "name": self.repository["name"].as_str().unwrap(),
        });
        let data = self.git_post.post(
            "repo/push/head", 
            data).await.unwrap();

        // eprintln!("cmd_list req repo/push/head result {:?}", data);
        // TODO no permision
        if data.err {
            eprintln!("push error {:?}", data.msg.unwrap());
            std::process::exit(1);
        }
        let remote_refs = data.data.unwrap()["refs"]
            .as_array()
            .map(|v| v.to_owned()).unwrap();

        if remote_refs.is_empty() {
            println!("0000 HEAD");
            println!();
            return 
        }

        for remote_ref_value in remote_refs {
            // eprintln!("remote_ref_value  {:?} ", remote_ref_value);
            let remote_ref_hash = remote_ref_value["hash"].as_str().unwrap();
            let remote_ref_name = remote_ref_value["ref_name"].as_str().unwrap();
            eprintln!("remote_ref_value  {} {} ", remote_ref_hash, remote_ref_name);


            if remote_ref_value.as_str() == Some("") {
                println!("0000 HEAD");
                println!();
                return
            }

            let message = format!("{} {}", remote_ref_hash, remote_ref_name);
            self.push_head_remote_ref_hash = Some(remote_ref_hash.to_string());
            println!("{}", message);
            eprintln!("<< {}", message);
            // let result = git_rev_list_objects(self.current_dir.clone(), param).unwrap();
        }
        println!();
    }

    fn _parse_ref(&self, push_ref: &str) -> BuckyResult<(bool, String, String, String)> {
        let force = push_ref.starts_with('+');
        eprintln!("cmd push is force {:?}", force);

        let mut split = push_ref.split(':');
    
        let src_ref = split.next().unwrap();
        let src_ref = if force { &src_ref[1..] } else { src_ref };
        let dst_ref = split.next().unwrap();
        eprintln!("cmd push ref defail ,src {}, dst {}", src_ref, dst_ref);
        if src_ref != dst_ref {
            eprintln!("src_ref != dst_ref");
            std::process::exit(1);
        }

        let branch = src_ref.clone().split("/").last().unwrap();
        let local_sha = git_rev_parse(self.current_dir.clone(), src_ref)?;
        eprintln!("cmd push local_ref {} {} {} ", src_ref, local_sha, branch);

        Ok((
            force,
            branch.to_string(),
            local_sha,
            dst_ref.to_string(),
        ))
    }

    pub fn cmd_complete(&self) {
        eprintln!("complete");
        std::process::exit(0);
    }
    pub fn cmd_unknown(&self) {
        eprintln!("unknown command");
        std::process::exit(0);
    }
}

