use cyfs_base::*;
use std::path::{PathBuf, Path};
use std::process::{Command, Output,Stdio};
use std::io::prelude::*;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ffi::OsStr;
use log::*;

use crate::*;


pub fn git_exec_base_command<I, S, P: AsRef<Path>> (repo_dir: P, args: I) -> Command
where
    I: std::fmt::Debug,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{

    let mut command = Command::new("git");
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    // let tmp_args: Vec<&str> = vec![];
    // for arg in args {
    //     tmp_args.push(arg.as_ref().to_os_string().into())
    // }
    info!("git exec: {:?}", args);
    command.args(args).current_dir(repo_dir);
    command
}

pub fn git_exec_base_output<I, S, P: AsRef<Path>> (repo_dir: P, args: I)-> BuckyResult<Output>
where
    I: std::fmt::Debug,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{

    let mut command = git_exec_base_command(repo_dir, args);
    let output = command.output().map_err(|e| {
        BuckyError::new(BuckyErrorCode::ExecuteError, format!("git exec error {:?}", e))
    })?;
    Ok(output)
}

pub fn git_exec_base<I, S, P: AsRef<Path>> (repo_dir: P, args: I) ->  BuckyResult<String>
where
    I: std::fmt::Debug,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = git_exec_base_output(repo_dir, args)?;
    if !output.status.success() {
        let msg = format!("git exec error: {:?}", String::from_utf8(output.stderr).unwrap());
        return Err(BuckyError::new(BuckyErrorCode::InternalError, msg))
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    Ok(stdout)
}




#[derive(Serialize, Deserialize)]
pub struct GitRef {
    pub ref_name: String,
    pub branch: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitFileCommitMessage  {
    pub commit: String, 
    pub author: String, 
    pub date: String, 
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitFileLineContent  {
    pub line: i32, 
    pub content: String, 
}



#[derive(Serialize, Deserialize, Debug)]
pub struct CommitDiffResult {
    pub file_name: String,
    pub file_content: Vec<CommitDiffResultLine>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CommitDiffResultLine {
    pub left_line: Option<i32>,
    pub right_line: Option<i32>,
    pub diff_type: String,
    pub content: String,
    pub is_tag: bool,
}

#[derive(Debug, Default)]
struct GitDiffLineCount {
    left: i32,
    right: i32,
    left_count: i32,
    right_count: i32,
    left_totle: i32,
    right_totle: i32,
}



pub fn git_init(repo_dir: PathBuf) -> BuckyResult<Output> {
    git_exec_base_output(repo_dir, ["init", "--bare"])
}

/// git_show_ref
pub fn git_show_target_branch_ref(repo_dir: PathBuf, branch: &str) -> BuckyResult<String> {
    // Limit to "refs/heads"
    // Only show the SHA-1 hash
    let stdout = git_exec_base(
        repo_dir, 
        ["show-ref", "--heads", "-s", branch])?;
    let hash = stdout
        .lines().next()
        .map(|x| x.to_owned()).unwrap();
    // let hash = stdout.lines().next().map(|line| line.split_whitespace().nth(0).unwrap().to_string()).unwrap();
    Ok(hash)
}

pub fn git_rev_parse(repo_dir: PathBuf, rev: &str) -> BuckyResult<String> {
    let result = git_exec_base(
        repo_dir, 
        ["rev-parse", rev]
    )?;
    Ok(result.trim().to_string())
}



/// git_show_ref
pub fn git_show_ref(repo_dir: PathBuf) -> BuckyResult<Vec<GitRef>> {
    let stdout = git_exec_base(repo_dir, ["show-ref"]).unwrap();
    let lines = stdout.lines();

    let mut refs:Vec<GitRef> = Vec::new();
    for line in lines {
        let contents: Vec<&str> = line.split_whitespace().collect();
        // e.g aa82b299bd0f8bd7337ca0a849e5f0950c45f192 refs/heads/master 
        //  ---> {name: master, ref_name: refs/heads/master, hash: aa82b299bd0f8bd7337ca0a849e5f0950c45f192}
        if contents.len() > 1 {
            let ref_name = contents[1].to_string();
            let branch = ref_name.split("/").last().unwrap().to_string();
            refs.push(GitRef{
                hash: contents[0].to_string(),
                ref_name: ref_name,
                branch: branch,
            })
        }
    }
    Ok(refs)
}


/// git_config_quotepath
/// git config 
/// 让git show能正常显示中文文件名字
pub fn git_config_quotepath(repo_dir: PathBuf) -> String {
    let stdout = git_exec_base(repo_dir, ["config", "core.quotepath", "false"]).unwrap();
    stdout
}


/// git_rev_list_objects
/// 获取commit_id 之间比对差异的object列表
pub fn git_rev_list_objects(repo_dir: PathBuf, rev_list_params: &str) ->  BuckyResult<String> {
    let stdout = git_exec_base(repo_dir, ["rev-list", "--objects", rev_list_params]);
    
    if let Err(err) = stdout {
        let err_msg = format!("git rev-list --object failed: {} {:?}", rev_list_params, err);
        error!("{}", err_msg);
        Err(BuckyError::new(BuckyErrorCode::Failed, err_msg))
    } else {
        let stdout = stdout.unwrap();
        let result = stdout.lines().filter_map(|v| {
            if v.len() > 0 {
                v.split_whitespace().next().map(|v| v.to_string())
            } else {
                None
            }
        }).collect::<Vec<String>>().join("\n");
        Ok(result)
    } 
}


/// git_pack_objects_to_file
/// 将object列表 pack 到文件中. 并返回文件路径
pub fn git_pack_objects_to_file(repo_dir: PathBuf, objects: String)  ->  BuckyResult<String> {
    let mut pack_target = cyfs_util::get_cyfs_root_path();
    pack_target.push("data");
    pack_target.push("app");
    pack_target.push(dec_id().to_string());
    pack_target.push(APP_REPOSITORY_PACK_DIR);
    if let Err(e) = std::fs::create_dir_all(&pack_target) {
        error!(
            "RepositoryHelper repo_dir create app data dir failed! dir={}, err={}",
            pack_target.display(),
            e
        );
    }
    // let pack_target = cyfs_util::get_app_data_dir(APP_REPOSITORY_PACK_DIR);
    let pack_target = pack_target.join("pack");  // add file prefix
    let pack_target = pack_target.to_str().unwrap();
    info!("pack_target {:?} ",pack_target);

    // 这个生成出来的pack文件， 不能直接放到/cyfs/tmp目录，不支持设备间的挂载目录（fatal: unable to rename temporary pack file: Invalid cross-device link)
    // /cyfs/tmp目录在linux环境下是主机挂载进来的目录
    // git exec pack object ---> file
    let mut command = git_exec_base_command(
        repo_dir.clone(),
        ["pack-objects", "--all-progress", pack_target]);
    let mut child =  command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(objects.as_bytes())?;
    drop(child_stdin);
    info!("pack ok , drop the stdin");

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let hash = stdout.trim_end().to_string();
    let target_file = format!("{}-{}.pack", pack_target, hash);
    info!("git pack object result {:?} ",target_file);
    Ok(target_file)
}

pub fn git_unpack_objects(repo_dir: PathBuf, pack_file_path: PathBuf) {
    let f = File::open(pack_file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer).unwrap();


    let mut command = git_exec_base_command(
        repo_dir.clone(),
        ["unpack-objects"]);
    let process = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    match process.stdin.unwrap().write_all(&buffer) {
        Err(err) => eprintln!("write_all buffer to unpack object failed: {}", err),
        Ok(_) => {},
    }
    // info!(" write_all");
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => eprintln!("couldn't read wc stdout: {}", why),
        Ok(_) => {},
    }
    // info!("unpack ok");
}

// git_unbundle_and_unpack_objects
// in service: push handler
pub fn git_unbundle_and_unpack_objects(repo_dir: PathBuf, bundle_file: &str, branch: &str, ref_hash: &str) -> BuckyResult<()> {
    let _stdout = git_exec_base(
        repo_dir.clone(), 
        ["fetch", bundle_file, branch])?;

    git_update_ref(repo_dir, branch, ref_hash);
    Ok(())
}

pub fn git_bundle_create(repo_dir: PathBuf, bundle_target: &str, git_rev_list_arg: &str) -> BuckyResult<()> {
    let _stdout = git_exec_base(
        repo_dir, 
        ["bundle", "create", bundle_target, git_rev_list_arg])?;
    Ok(())
}


pub fn git_update_ref(repo_dir: PathBuf, branch: &str, ref_hash: &str) {
    let name = format!("refs/heads/{}", branch);
    // info!("git update-ref {} {}", &name, ref_hash);
    let output = git_exec_base_output(repo_dir,["update-ref", &name, ref_hash]).unwrap();
    debug!("git update-ref result {:?}", output);
}




/// git_ls_tree_file
/// 查找blob文件对应的hash id
pub fn git_ls_tree_file(repo_dir: PathBuf, branch: &str, file_path: &str) -> BuckyResult<GitLsTreeResultTreeData> {
    let stdout = git_exec_base(repo_dir, ["ls-tree", branch, file_path]).unwrap();
    println!("git log {:?}", stdout);
    let result = git_ls_object_parse(stdout.as_str())?;
    Ok(result)
}

/// git_cat_file_size
/// cat 文件大小
pub fn git_cat_file_size(repo_dir: PathBuf, blob: &str) -> BuckyResult<i64> {
    let mut stdout = git_exec_base(repo_dir, ["cat-file", "-s", blob]).unwrap();
    println!("git cat file {:?}", stdout);
    stdout.truncate(stdout.len() - 1);
    let size: i64 = stdout.parse().unwrap();
    Ok(size)
}



/// git_show_file_content
/// 文件内容
pub fn git_show_file_content(repo_dir: PathBuf, blob: &str) -> BuckyResult<Vec<GitFileLineContent>> {
    info!("git git_show_file_content {:?}", blob);
    let stdout = git_exec_base(repo_dir.clone(), ["show", blob]).unwrap();


    let lines = stdout.lines();
    let mut index = 0;
    let mut content:Vec<GitFileLineContent> = vec![];
    for line in lines {
        index = index + 1;
        println!("git show {:?}", line);
        content.push(GitFileLineContent{
            line: index,
            content: line.to_string(),
        })
    }
    Ok(content)
}

/// git_log_commit_message
/// 文件的git log message 信息
pub fn git_log_commit_message(repo_dir: PathBuf, branch: &str, file_path: &str) -> BuckyResult<GitFileCommitMessage> {
    info!("git_log_commit_message params {:?} , file_path: {:?}", branch, file_path);
    
    
    // %H  ==> full hash
    let stdout = git_exec_base(repo_dir, ["log", "--max-count=1", "--pretty=format:%H;%an;%ad;%s", branch,"--", &file_path]);

    if stdout.is_ok() {
        let stdout = stdout.unwrap();
        let words: Vec<&str>  = stdout.split(";").collect();
        if words.len() < 4 {
            info!("git_log_commit_message failed {:?}", words);
            return Err(BuckyError::new(BuckyErrorCode::Failed, "git_log_commit_message"));
        }
        info!("git log single file[{:?}] message {:?}", file_path, words);
        let commit_message = GitFileCommitMessage{
            commit: words[0].to_string(),
            author:  words[1].to_string(),
            date:  words[2].to_string(),
            message:  words[3].to_string(),
        };
        info!("git log {:?}", commit_message);
        return Ok(commit_message)
    }

    Err(BuckyError::new(BuckyErrorCode::Failed, "git log commit message error"))
}

/// git_log_last_commit_message
/// 最后一次commit的信息
pub fn git_log_last_commit_message(repo_dir: PathBuf, branch: &str) -> BuckyResult<GitFileCommitMessage> {
    let stdout = git_exec_base(repo_dir, ["log", "--max-count=1", r#"--pretty=format:%h;%an;%ad;%s"#, branch, "--" ]);
    if stdout.is_ok() {
        let stdout = stdout.unwrap();
        let words: Vec<&str>  = stdout.split(";").collect();
        println!("git log {:?}", words);
        let commit_message = GitFileCommitMessage{
            commit: words[0].to_string(),
            author:  words[1].to_string(),
            date:  words[2].to_string(),
            message:  words[3].to_string(),
        };
        println!("git log {:?}", commit_message);
        return Ok(commit_message)
    }

    Err(BuckyError::new(BuckyErrorCode::Failed, "git log commit message error"))
}

/// git_log_last_commit_message
/// 计算 commit 数量
pub fn git_rev_list_count(repo_dir: PathBuf, branch: &str) -> BuckyResult<i32> {
    let stdout = git_exec_base(repo_dir, ["rev-list", "--count", branch]);
    let mut stdout =stdout.unwrap();
    stdout.truncate(stdout.len() - 1);
    info!("git git_rev_list_count {:?}", stdout);
    let count: i32 = stdout.parse().unwrap();
    Ok(count)
}




pub fn git_lastest_commit_oid(path: PathBuf, branch: &str) ->  BuckyResult<String> {
    let ref_head_target = format!("refs/heads/{}", branch);
    let result = git_exec_base(path, ["show-ref", "-s", &ref_head_target])?;
    let lastest_oid = result.trim();
    Ok(lastest_oid.to_string())
}

/// git_commits
/// 
pub fn git_commits(repo_dir: PathBuf, branch: &str) -> BuckyResult<Vec<GitCommit>> {
    let result = git_exec_base(repo_dir.clone(), ["log", "--format='%H'", branch])?;
    let commits: Vec<GitCommit> = result.lines().map(|log| {
        debug!("commit oid {}", log);
        let oid = log.trim().trim_matches('\'');
        let commit = git_read_commit_object(repo_dir.clone(), oid).expect("parse commit info failed");
        commit
    }).collect();
    debug!("git read commits of branch {}, count {} ", branch, commits.len());
    Ok(commits)
}


/// get_diff_file_name
/// 匹配获取 文件名字
fn get_diff_file_name(input: &str) -> BuckyResult<String> {
    // 要支持名字带空格和中文
    // "diff --git a/diff1.txt b/diff1.txt"
    let re = Regex::new(r"diff[ ]--git[ ]a/(?P<name>.+)[ ]b/").unwrap();
    let caps = re.captures(input);
    if caps.is_none() {
        return Err(BuckyError::new(BuckyErrorCode::Failed, "git diff file name error"))
    }
    let result = caps
        .and_then(|cap| cap.name("name"))
        .map(|name| name.as_str().to_string()).unwrap();
    Ok(result)
}

/// get_diff_line
/// 匹配获取 diff的文件起始行号等信息
fn get_diff_line(input: &str) -> BuckyResult<GitDiffLineCount> {
    // "@@ -3,4 +3,4 @@"
    let re = Regex::new(r"@@[ ][\+\-](?P<left>\d+)(,(?P<left2>\d+))?[ ][\+\-](?P<right>\d+)(,(?P<right2>\d+))?[ ]").unwrap();
    let caps = re.captures(input);
    if caps.is_none() {
        return Err(BuckyError::new(BuckyErrorCode::Failed, "git diff line number error"))
    }
    let result = caps.unwrap();
    let left = result.name("left").unwrap().as_str().parse::<i32>().unwrap();
    let left2 = result.name("left2").map(|x| x.as_str().parse::<i32>().unwrap()).unwrap_or(0);
    // let left2 = result.name("left2").unwrap().as_str().parse::<i32>().unwrap();
    let right = result.name("right").unwrap().as_str().parse::<i32>().unwrap();
    let right2 = result.name("right2").map(|x| x.as_str().parse::<i32>().unwrap()).unwrap_or(0);
    // let right2 = result.name("right2").unwrap().as_str().parse::<i32>().unwrap();
    Ok(GitDiffLineCount{
        left,
        right,
        left_totle: left2,
        right_totle: right2,
        left_count: left,
        right_count: right,
    })
}


// 逐处理 diff的内容行
impl CommitDiffResultLine {
    fn tag_line(content: &str) -> CommitDiffResultLine {
        CommitDiffResultLine {
            left_line: None,
            right_line: None,
            diff_type: "tag".to_string(),
            content: content.to_string(),
            is_tag: true,
        }
    }
    fn code_line(content: &str, line_result: &mut GitDiffLineCount) -> CommitDiffResultLine {
        let mut result = CommitDiffResultLine {
            left_line: None,
            right_line: None,
            diff_type: "".to_string(),
            content: "".to_string(),
            is_tag: false,
        };

        if content.starts_with("+") {
            result.diff_type = "add".to_string();
            result.content = content.to_string().trim_start_matches("+").to_string();
        } else if content.starts_with("-") {
            result.diff_type = "remove".to_string();
            result.content = content.to_string().trim_start_matches("-").to_string();
        } else {
            result.content = content.to_string().trim_start().to_string(); // 行首的空格
        }

        if line_result.left_count < line_result.left_totle + line_result.left && result.diff_type != "add" {
            result.left_line = Some(line_result.left_count);
            line_result.left_count += 1;
        }

        let is_left_zero = line_result.left_totle == 0 && line_result.left == 0;

        if (line_result.right_count < line_result.right_totle + line_result.right && result.diff_type != "remove") || is_left_zero {
            result.right_line = Some(line_result.right_count);
            line_result.right_count += 1;
        }
        // println!("@@ line_result {:?}", line_result);    
        result
    }
}

/// git_show_commit_diff
/// diff 信息 结构化
pub fn git_show_commit_diff(repo_dir: PathBuf, parent: &str, commit_id: &str ) -> BuckyResult<Vec<CommitDiffResult>>  {
    // first commit 没有parent, 直接一个oid就可以
    let output = if parent == "" {
        git_exec_base_output(repo_dir, ["show", &commit_id])
    } else {
        let diff_arg = format!("{}..{}", parent, commit_id);
        git_exec_base_output(repo_dir, ["diff", &diff_arg])
    }?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut lines = stdout.lines();

    let mut data: Vec<CommitDiffResult> = vec![];
    // println!("git show diff {:?}", stdout);

    let mut file_name_line = lines.next().unwrap();
    loop {
        // let start_line = start_line.unwrap();
        if file_name_line.starts_with("diff --git") {

            let file_name = get_diff_file_name(file_name_line)?;
            println!("get_diff_file_name {:?}", file_name);

            let mut file_content: Vec<CommitDiffResultLine> = vec![];
            let mut result = GitDiffLineCount::default();
            // let mut right_line = 0;
            loop {
                let line = lines.next();
                if line.is_none() {
                    file_name_line = "";
                    break
                }
                let content = line.unwrap();
                if content.starts_with("diff --git") {
                    file_name_line = content;
                    break
                } else if content.starts_with("@@ ") {
                    result = get_diff_line(content)?;
                    println!("@@ line_result {:?}", result);    
                    file_content.push(CommitDiffResultLine::tag_line(content));
                } else if content.starts_with("new file mode") 
                || content.starts_with("index ")
                || content.starts_with("--- ")
                || content.starts_with("--- ")
                || content.starts_with("\\ No newline at end of file")
                || content.starts_with("+++ ") {
                    continue
                } else {
                    file_content.push(CommitDiffResultLine::code_line(content, &mut result));
                }
            }

            data.push(CommitDiffResult{
                file_name,
                file_content,
            });

        } else {
            let next =  lines.next();
            if next.is_none() {
                break
            }
            file_name_line = next.unwrap();
        }
    }
    // println!("git git_show_commit {:#?}", data);
    Ok(data)
}

pub fn git_compare_no_merge(repo_dir: PathBuf, origin_branch: &str, target_branch: &str) -> BuckyResult<Vec<serde_json::Value>>  {
    let compare_param = format!("{}..{}", target_branch, origin_branch);
    let stdout = git_exec_base(repo_dir,["log", &compare_param, "--oneline", "--no-merges"]).unwrap();

    let result = stdout.lines().filter_map(|v| {
        if v.len() > 0 {
            let mut r = v.split_whitespace();
            let commit = r.next().map(|v| v.to_string());
            let message = r.collect::<Vec<&str>>().join(" ");
            Some(json!({
                "commit": commit.unwrap(),
                "message": message,
            }))
        } else {
            None
        }
    }).collect::<Vec<serde_json::Value>>();

    Ok(result)
}

pub fn git_diff_stat(repo_dir: PathBuf, origin_branch: &str, target_branch: &str) -> BuckyResult<Vec<serde_json::Value>>  {
    let compare_param = format!("{}..{}", target_branch, origin_branch);
    let stdout = git_exec_base(repo_dir,["diff", &compare_param, "--stat=10000"]).unwrap();
    let mut data: Vec<serde_json::Value> = Vec::new();
    let _result = stdout.lines().for_each(|line| {
        let line = line.split("|");
        if line.clone().count() > 1 {
            let name = line.clone().nth(0).unwrap().trim();
            let number = line.clone().nth(1).unwrap().replace("+", "");
            let number = number.trim();
            data.push(json!({
                "file_name": name,
                "count": number,
            }));
        } 
    });
    //println!("git result {:?}", result);
    Ok(data)  
}

pub fn git_merge_diff_file(repo_dir: PathBuf, origin_branch: &str, target_branch: &str, file_name: &str) -> BuckyResult<Vec<CommitDiffResultLine>>{
    let compare_param = format!("{}..{}", target_branch, origin_branch);

    let stdout = git_exec_base(repo_dir,["diff", &compare_param, "--", file_name]).unwrap();

    let mut lines = stdout.lines();
    let mut file_content: Vec<CommitDiffResultLine> = vec![];
    let mut result = GitDiffLineCount::default();
    loop {
        let line = lines.next();
        if line.is_none() {
            break
        }
        let content = line.unwrap();
        if content.starts_with("diff --git") {
            continue
        } else if content.starts_with("@@ ") {
            result = get_diff_line(content)?;
            println!("@@ line_result {:?}", result);    
            file_content.push(CommitDiffResultLine::tag_line(content));
        } else if content.starts_with("new file mode") 
        || content.starts_with("index ")
        || content.starts_with("--- ")
        || content.starts_with("--- ")
        || content.starts_with("\\ No newline at end of file")
        || content.starts_with("+++ ") {
            continue
        } else {
            file_content.push(CommitDiffResultLine::code_line(content, &mut result));
        }
    }
    // println!("content {:?}", file_content);
    Ok(file_content)
}




// ---------------------
// test
#[cfg(test)]
mod main_tests {
    use super::*;
    use cyfs_debug::*;
    
    #[async_std::test]
    async fn test_git_commits() {
        cyfs_debug::CyfsLoggerBuilder::new_app("cyfs-git-test")
        .level("info")
        .console("info")
        .enable_bdt(Some("error"), Some("error"))
        .module("cyfs_lib", Some("error"), Some("error"))
        .build()
        .unwrap()
        .start();
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle001\20220811"#);
        git_commits(repo_dir, "master").unwrap();
    }

    #[async_std::test]
    async fn test_git_show_commit() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\aaa"#);
        git_show_commit_diff(repo_dir, "", "a7d2f4c7383208ec115fbe6024aee058c9a0f62f").unwrap();
    }

    #[async_std::test]
    async fn test_git_show_commit2() {
        let repo_dir= PathBuf::from("/cyfs/data/app/9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r/cyfs-git-repos/alex_huawei/0509");
        git_show_commit_diff(repo_dir, "", "041e2b2691b9faa92c4a5091b6c4a46f13881a43").unwrap();
    }


    #[async_std::test]
    async fn test_reg() {
        let input = "diff --git a/services/cyfs-git-base/src/git.rs b/services/cyfs-git-base/src/git.rs";
        let result = get_diff_file_name(input);

        println!("result {:?}", result);
        assert_eq!(result.unwrap(), "services/cyfs-git-base/src/git.rs");
    }

    #[async_std::test]
    async fn test_diff_reg_cn() {
        CyfsLoggerBuilder::new_app(CYFS_GIT_DEC_APP_NAME)
        .level("info")
        .console("info")
        .enable_bdt(Some("off"), Some("off"))
        // .module("non-lib", Some("off"), Some("off"))
        .build()
        .unwrap()
        .start();
        let input = "diff --git a/readme - 副本 (2).md b/readme - 副本 (2).md";
        let result = get_diff_file_name(input);

        info!("result {:?}", result);
        assert_eq!(result.unwrap(), "readme - 副本 (2).md");
    }


    #[async_std::test]
    async fn test_git_ls_tree() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\0629003"#);
        let result = git_ls_tree_path(repo_dir, "master", "").unwrap();
        println!("result {:?}", result);
    }

    #[async_std::test]
    async fn test_git_rev_list_objects() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\0309"#);
        let result = git_rev_list_objects(repo_dir, "64f645d294f3916b25588f20a7ce24df45720739").unwrap();
        println!("result {:?}", result);
    }


    #[async_std::test]
    async fn test_git_diff_stat() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\0309"#);
        let result = git_diff_stat(repo_dir, "test","master").unwrap();
        println!("result {:?}", result);
    }

    #[async_std::test]
    async fn test_git_merge_diff_file() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\0309"#);
        let result = git_merge_diff_file(repo_dir, "test","master", "main.js").unwrap();
        println!("result {:?}", result);
    }

    #[async_std::test]
    async fn test_git_do_merge() {
        let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle"#);
        let output = Command::new("cp")
            .args(["0309", "test", "-R"])
            .current_dir(repo_dir)
            .output().unwrap();

        println!("result {:?}", output);

    }


    // #[async_std::test]
    // async fn test_git_generate_pack() {
    //     let repo_dir= PathBuf::from(r#"E:\app\cyfs-git-rust"#);
    //     let result = git_generate_pack(
    //         repo_dir, 
    //         r"E:\app\cyfs-git-rust\.git\cyfs-git",
    //         "master"
    //     );
    //     println!("result {:?}", result);
    // }

    #[async_std::test]
    async fn test_git_generate_bundle() {
        let repo_dir= PathBuf::from(r#"E:\app\cyfs-git-rust"#);
        let result = git_bundle_create(
            repo_dir, 
            r"E:\app\cyfs-git-rust\.git\master.bundle",
            "master"
        );
        println!("result {:?}", result);
    }

    #[async_std::test]
    async fn test_git_unpack_objects() {
        let repo_dir= PathBuf::from(r#"D:\app\clone\test"#);
        let pack_file_path= PathBuf::from(r"D:\app\clone\test\.git\objects\pack\pack-0a126038e26417b7692bd3c0c275376e29148346.pack");
        let result = git_unpack_objects(
            pack_file_path,
            repo_dir, 
        );
        println!("result {:?}", result);
    }

    #[async_std::test]
    async fn test_git_unbundle_and_unpack_objects() {
        let repo_dir= PathBuf::from(r#"D:\app\clone\test"#);
        let bundle_file = r"E:\app\cyfs-git-rust\.git\master.bundle";
        // let pack_file_path= PathBuf::from(r"D:\app\clone\test\.git\objects\pack\pack-0a126038e26417b7692bd3c0c275376e29148346.pack");
        let result = git_unbundle_and_unpack_objects(
            repo_dir, 
            bundle_file,
            "master",
            "37052e464bb5b5bc9559d404cd3fec56c0ad0a09",
        );
        println!("result {:?}", result);
    }


    // for linux
    #[async_std::test]
    async fn test_git_log_commit_message() {
        // 
        let repo_dir= PathBuf::from(r#"/root/app/cyfs-git-rust"#);
        let result = git_log_commit_message(repo_dir, "master", "README.md");
        println!("result {:?}", result);

    }

}