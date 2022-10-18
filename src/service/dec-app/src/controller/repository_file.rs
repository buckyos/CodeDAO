use cyfs_lib::*;
use cyfs_base::*;
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use cyfs_git_base::*;
use log::*;


use std::path::Path;
use std::ffi::OsStr;
use tokei::LanguageType;



#[derive(Serialize, Deserialize)]
struct RequestRepositoryFile {
    // id: String,
    author_name: String,
    name: String,
    path: String,
    branch: String,
    hash: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TreeFileMessage  {
    /// 文件名字
    file: String,  
    file_relative_path: String,
    file_type: String,
    commit: String, 
    author: String, 
    date: String, 
    message: String,
}



/// # repository_file   
/// 文件树获取
pub async fn repository_file(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryFile = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    
    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }


    let file_path = if data.path == "/" { // 根路径 / 设置为 空""
        "".to_string()
    } else {
        data.path
    };
    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    let repo_dir = repository.repo_dir();
    info!("repo dir {:?}", repo_dir);

    let result = git_ls_tree_path(repo_dir.clone(), &data.branch, &file_path)?;
    info!("git_ls_tree_path {:?}", result);

    // 目录下的文件信息
    if result.file_type == "dir" {
        let mut dir_files_message: Vec<TreeFileMessage> = vec![];

        // tree method
        /* 
        let refs_hash = git_exec_base(repo_dir.clone(), ["show-ref", "-s", &format!("refs/heads/{}", data.branch)])?;
        let lastest_oid = refs_hash.trim();
        let mut treefile_helper = TreeFileHelper::new(
            Arc::clone(&ctx.stack),
            space.clone(),
            name.clone(),
            lastest_oid.to_string(),
            data.branch.clone(),
            file_path.clone(),
        ).await?;
        treefile_helper.prepare().await?;
        let result_main_file_map = treefile_helper.start().await?;
        for (key, value) in  result_main_file_map.clone().into_iter() {
            dir_files_message.push(TreeFileMessage{
                file: key.clone(),
                file_relative_path: key,
                file_type: value.file_type.clone(),
                commit: value.hash,
                author: value.author,
                date: value.date,
                message: value.message,
            })
        }*/

        // 1 可能可cached
        // 2 branch 支持branch或者hash
        for item in result.tree.unwrap() {
            // 子文件的相对路径
            // 如果 /<repo>/aaa/bbb.js ->结果就是 aaa/bbb.js
            let item_file_path = if &file_path == "" {
                item.file_name.clone()
            } else {
                format!("{}/{}", file_path, item.file_name)
            };

            // 获取文件git log信息
            let message = git_log_commit_message(repo_dir.clone(), &data.branch, &item_file_path)?;
            // println!("git_log_commit_message {:?}", message);
            dir_files_message.push(TreeFileMessage{
                file: item.file_name.clone(),
                file_relative_path: item_file_path,
                file_type: item.git_object_type.clone(),
                commit: message.commit,
                author: message.author,
                date: message.date,
                message: message.message,
            })
        }

        // file_tree排序：  文件夹 > 文件，然后再按首字母
        dir_files_message.sort_by(|a, b| {
            if a.file_type == b.file_type {
                a.file.cmp(&b.file)
            } else {
                if a.file_type == "blob" {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Less;
                }
            }
        });

        // check 文件列表中的readme.md文件，
        // 如果当前这一层文件里有readme，就获取blob并拿到内容
        let readme = {
            let index = dir_files_message.iter().position(|tree| tree.file.to_lowercase() == "readme.md");
            if index.is_some() {
                let index = index.unwrap();
                let target = &dir_files_message[index];
                // let execCommand = format!("git ls-tree -r ${} ${}", target.commit, target.file);

                // ls-tree file 要用相对路径。否则只会到repo root的readme.md
                let content = git_exec_base(repo_dir.clone(), ["ls-tree", "-r", &target.commit, &target.file_relative_path]);
                let content = content.unwrap();
                //  固定位数: 100644 blob e8990606abf9ac711afc07dc60d7061e3812042d    readme.md
                let blob: String = content.chars().skip(12).take(40).collect();


                let content = git_exec_base(repo_dir.clone(), ["cat-file", "-p", &blob]);
                let content = content.unwrap();
                json!({
                    "type": "md",
                    "content": content,
                })
            } else {
                json!({})
            }
        };

        return Ok(success(json!({
            "fileType": "dir",
            "dirData": {
                "data": dir_files_message,
                "readme": readme,
            }
        })))
    }

    // 非目录的逻辑
    // 文件类型的处理
    // 太大的不展开，不支持的类型不展开（rp，word等不支持展开），ext类型判断简单一点，直接依赖tokie。

    let tree_file_blob = git_ls_tree_file(repo_dir.clone(), &data.branch, &file_path)?;
    let size = git_cat_file_size(repo_dir.clone(), &tree_file_blob.file_hash)?;
    let message = git_log_commit_message(repo_dir.clone(), &data.branch, &file_path)?;

    // get file common return value
    let file_info_json = |big: bool, extension: bool, content: Option<Vec<GitFileLineContent>>| {
        json!({
            "fileType": "file",
            "fileData": {
                "bigFile": big,
                "notSupport": extension,
                "content": content,
                "info": {
                    "commit": message.commit,
                    "author": message.author,
                    "date": message.date,
                    "message": message.message,
                    "fileSize": size,
                },
            }
        })
    };


    // 检查ext是否是能展开的文件类型
    let ext = Path::new(&file_path)
        .extension()
        .and_then(OsStr::to_str);
    if ext.is_some() {
        info!("file path extension: {:?}", ext);
        let result = LanguageType::from_file_extension(ext.unwrap());
        if result.is_none() {
            return Ok(success(file_info_json(false, true, None)))
        }
    }


    // 如果大于 2mb 就不展示内容了
    if size > (2 << 20)  {
        return Ok(success(json!({
            "fileType": "file",
            "fileData": {
                "bigFile": true,
            }
        })))
    }

    let content = git_show_file_content(repo_dir.clone(), &tree_file_blob.file_hash);
    if content.is_err() {
        return Ok(success(file_info_json(false, true, None)))
    }
    
    Ok(success(file_info_json(false, false, Some(content.unwrap()))))
}