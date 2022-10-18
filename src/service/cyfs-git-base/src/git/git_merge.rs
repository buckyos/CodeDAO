use cyfs_base::*;
use std::path::PathBuf;
// use std::io::prelude::*;
use log::*;
use crate::*;



// git_merge
// 这里重点是在一个bare仓库处理合并。
// 核心三个步骤. read-tree,  write-tree, commit-tree
// 参考 https://stackoverflow.com/questions/7984986/git-merging-branches-in-a-bare-repository
pub fn git_merge(
    repo_dir: PathBuf,
    source: &str,
    target: &str,
    // _author: &str,
) -> BuckyResult<String>{
    // get hash 
    // info!("git merge get branch hash: ours:{} {:?}, theirs:{} {:?}",ours, ours_hash, theirs, theirs_hash);

    // 通过read-tree 在.git目录下面创建index
    git_exec_base(repo_dir.clone(), ["read-tree", "-i", "-m", &target, &source])?;
    info!("git merge create index file ok");
    

    let new_tree = git_exec_base(repo_dir.clone(), ["write-tree"])?;
    let new_tree = new_tree.trim();
    println!("{:?}", new_tree);

    // github's merge message :: Merge pull request #5 from alexsunxl/master
    let commit_message = format!("Merge from \"{}\"", source);
    let merge_result_hash = git_exec_base(
        repo_dir.clone(),
        ["commit-tree", new_tree.trim(), "-p", &target, "-p", &source, "-m", &commit_message])?;
    let merge_result_hash = merge_result_hash.trim();
    info!("git merge  commit tree result {:?}", merge_result_hash);

    // 更新ref
    git_update_ref(repo_dir.clone(), target, merge_result_hash);
    Ok(merge_result_hash.to_string())
}


#[cfg(test)]
mod main_tests {
    use super::*;
    #[async_std::test]
    async fn test_git_merge() {
        // let repo_dir= PathBuf::from(r#"C:\cyfs\data\app\cyfs-git-repos\sunxinle\test2"#);
        let repo_dir= PathBuf::from(r#"D:\app\aaa"#);
        // let r = git_merge(repo_dir, "test",  "master", "sunxinle");
        // println!("{:?}", r);
        
    }
}