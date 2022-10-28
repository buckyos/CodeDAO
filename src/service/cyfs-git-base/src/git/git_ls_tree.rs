use cyfs_base::*;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;
use async_std::task;
use log::*;
use std::sync::{Arc, Mutex};
use std::thread;
use tokei::{Config, LanguageType};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitLsTreeResult {
    file_path: String,
    pub file_type: String,
    pub tree: Option<Vec<GitLsTreeResultTreeData>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitLsTreeResultTreeData {
    pub file_mode: String,
    pub git_object_type: String,
    pub file_hash: String,
    pub file_name: String,
}

/// git_ls_object_parse
/// 解析git ls-tree的行结果
pub fn git_ls_object_parse(input: &str) -> BuckyResult<GitLsTreeResultTreeData> {
    // 这里要通过正则处理， 而不是split white，否则文件名字里带了空格会出错
    let re = Regex::new(r"(?P<mode>\d+)\s(?P<type>\S+)\s(?P<hash>\S+)\s+(?P<name>.+)").unwrap();
    let result = re.captures(input);
    if result.is_none() {
        error!("git ls-tree parse error input: {}", input);
        return Err(BuckyError::new(
            BuckyErrorCode::Failed,
            "parse ls-tree result error",
        ));
    }
    let result = result.unwrap();
    Ok(GitLsTreeResultTreeData {
        file_mode: result.name("mode").unwrap().as_str().to_string(),
        git_object_type: result.name("type").unwrap().as_str().to_string(),
        file_hash: result.name("hash").unwrap().as_str().to_string(),
        file_name: result.name("name").unwrap().as_str().to_string(),
    })
}

/// git_ls_tree_path
/// git 根据目录参数（仓库子路径），查询目录下的文件
pub fn git_ls_tree_path(
    repo_dir: PathBuf,
    branch: &str,
    file_path: &str,
) -> BuckyResult<GitLsTreeResult> {
    let param = format!("{}:{}", branch, file_path);
    info!("[{:?}] git ls-tree {}", repo_dir, param);
    // ls-tree 拿到当前tree下面的文件信息
    // 输出如下
    // 040000 tree dd8662beecd7569d49fc886ee9d08fa5f6f15adf    aaa
    // 100644 blob 97f3658355faa5f27aee3d08ac0f8276ce9ef4cc    index.js
    let stdout = git_exec_base(repo_dir, ["ls-tree", &param]);
    if stdout.is_ok() {
        let stdout = stdout.unwrap();
        let tree: Vec<GitLsTreeResultTreeData> = stdout
            .lines()
            .map(|line| git_ls_object_parse(line).unwrap())
            .collect();

        Ok(GitLsTreeResult {
            file_type: "dir".to_string(),
            file_path: file_path.to_string(),
            tree: Some(tree),
        })
    } else {
        // ls-tree 命令出错了，那个这个hash就是个非tree对象，目前先当文件blob类型处理
        Ok(GitLsTreeResult {
            file_type: "file".to_string(),
            file_path: file_path.to_string(),
            tree: None,
        })
    }
}

pub fn stat_repo_with_cache(
    repo_dir: PathBuf,
    branch: &str,
    cache_key: &str,
) -> BuckyResult<HashMap<LanguageType, usize>> {
    info!("languages cache key {}", cache_key);
    let cache_value = CyfsGitCache::get(&cache_key)?;
    if cache_value.is_some() {
        let cache_value = cache_value.unwrap();
        info!(
            "get languages cache: {} , value: {}",
            cache_key, cache_value
        );
        let languages: HashMap<LanguageType, usize> =
            serde_json::from_str(&cache_value).map_err(transform_err)?;
        return Ok(languages);
    } else {
        let languages = stat_repo(repo_dir, branch)?;
        let value = serde_json::to_string(&languages).map_err(transform_err)?;
        CyfsGitCache::put(&cache_key, &value)?;

        let languages: HashMap<LanguageType, usize> =
            serde_json::from_str(&value).map_err(transform_err)?;
        return Ok(languages);
    }
}

pub fn stat_repo(
    repo_dir: PathBuf,
    branch: &str,
) -> BuckyResult<Arc<Mutex<HashMap<LanguageType, usize>>>> {
    let now = std::time::Instant::now();

    // -r 获得 分支下所有文件的blob  （-r recurse into subtrees）
    let stdout = git_exec_base(repo_dir.clone(), ["ls-tree", "-r", branch]).unwrap();
    let tree: Vec<GitLsTreeResultTreeData> = stdout
        .lines()
        .map(|line| git_ls_object_parse(line).unwrap())
        .collect();

    let repo_dir = Arc::new(repo_dir);
    let config = Arc::new(Config::default());
    let languages: Arc<Mutex<HashMap<LanguageType, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut tasks = Vec::with_capacity(tree.len());
    for item in tree {
        if item.git_object_type == "blob" {
            let repo_dir = Arc::clone(&repo_dir);
            let config = Arc::clone(&config);
            let languages = Arc::clone(&languages);
            tasks.push(task::spawn(async move {
                let now = std::time::Instant::now();
                let file_name = item.file_name.clone();
                let file_hash = item.file_hash.clone();
                info!("task[{}] start", file_name);

                stat_file(&file_name, &file_hash, &*config, repo_dir, languages);

                let elapsed = now.elapsed();
                info!("task[{}] end, cost {:.2?}", file_name, elapsed);
            }))
        }
    }

    // 获得task 结果
    task::block_on(async {
        for t in tasks {
            t.await;
        }
    });

    let elapsed = now.elapsed();
    info!("repo_dir:{:?} stat Elapsed: {:.2?}", repo_dir, elapsed);
    Ok(languages)
}

pub fn stat_file(
    file_name: &str,
    file_hash: &str,
    config: &Config,
    repo_dir: Arc<PathBuf>,
    languages: Arc<Mutex<HashMap<LanguageType, usize>>>,
) {
    let result = Path::new(file_name).extension();
    if result.is_none() {
        return;
    }
    let ext = result.unwrap().to_str().unwrap();
    let language_type = LanguageType::from_file_extension(ext);
    if language_type.is_none() {
        return;
    }
    let language_type = language_type.unwrap();

    let output = git_exec_base_output(&*repo_dir, ["show", file_hash]).unwrap();

    let code_state = language_type.parse_from_slice(output.stdout, &*config);
    info!(
        "[{}] {:?} code_state {:?}. thread: {:?}",
        file_name,
        language_type,
        code_state,
        thread::current().id()
    );

    // // 累计
    let languages = &*languages;
    let mut languages = languages.try_lock().unwrap();
    languages
        .entry(language_type)
        .and_modify(|e| *e += code_state.code)
        .or_insert(code_state.code);
}

// test
#[cfg(test)]
mod main_tests {
    use super::*;
    use cyfs_debug::*;

    #[async_std::test]
    async fn test_git_ls_tree_all_stat() {
        CyfsLoggerBuilder::new_app(CODE_DAO_SERVICE_NAME)
            .level("info")
            .console("info")
            .enable_bdt(Some("off"), Some("off"))
            .build()
            .unwrap()
            .start();
        let repo_dir = PathBuf::from(r#"E:\app\cyfs-git-rust"#);
        let branch = "master";
        let languages = stat_repo(repo_dir, branch);

        // TODO markdown 为啥是0行？
        info!("languages {:?}", languages);
    }
}
