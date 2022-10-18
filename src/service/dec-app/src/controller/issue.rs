use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct RequestIssueNew {
    author_name: String,
    name: String,
    title: String,
    content: String,
}

type RequestRepositoryIssueList = super::RequestRepositoryDelete;

#[derive(Serialize, Deserialize, Debug)]
struct RequestIssueDetail {
    author_name: String,
    name: String,
    issue_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestIssueClose {
    author_name: String,
    name: String,
    issue_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestIssueComment {
    author_name: String,
    name: String,
    issue_id: String,
    content: String,
}

/// # issue_create    
/// 创建issue
pub async fn issue_create(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestIssueNew = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    // 获取issue count id
    // let db = CyfsGitDatabase::instance().await.unwrap();
    // let issue_id = db.count_issue_id(&data.author_name, &data.name).await?;

    // info!("current issue id {}, title {} ",issue_id, data.title);
    // 获取 用户名字
    let user_name = get_user_name_by_owner(&ctx.caller.to_string()).await?;
    info!("get user name[{}  {}] ", ctx.caller.to_string(), user_name);

    let issue_id = "#";
    // 创建issue对象
    let issue = Issue::create(
        ctx.caller,
        data.title,
        data.content,
        "open".to_string(),
        "main".to_string(),
        user_name,
        issue_id.to_string(),
    );
    put_object(&ctx.stack, &issue).await?;

    let env = ctx.stack_env().await?;
    let object_id = issue.desc().calculate_id();
    let issue_topic_key = issue_topic_key(&data.author_name, &data.name, &object_id.to_string());
    let r = env.insert_with_path(&issue_topic_key, &object_id).await?;
    info!("insert_with_key: {:?}", r);

    // db.insert_issue_topic(&data.author_name, &data.name, issue.user_name(), issue_id, issue.title(),issue.content(), issue.status(), issue.issue_type()).await?;
    // info!("insert issue_topic to sqlite: ");

    let _root = env.commit().await;

    Ok(success(json!({"message": "ok",})))
}

/// # repository_issue_list    
/// 仓库的issue 列表
pub async fn repository_issue_list(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryIssueList =
        serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    let env = ctx.stack_single_env().await?;
    let base_key = issue_map_base_path(&data.author_name, &data.name);

    // 获取issue count id
    // let db = CyfsGitDatabase::instance().await.unwrap();
    // let result = db.select_issue_topic(&data.author_name, &data.name).await?;

    let mut result: Vec<serde_json::Value> = Vec::new();
    let mut close_count = 0;
    let mut open_count = 0;
    if env.load_by_path(&base_key).await.is_ok() {
        loop {
            let ret = env.next(10).await?;
            if ret.len() == 0 {
                break;
            }

            for item in ret {
                println!("item: {:?}", item);
                let (_, id) = item.into_map_item();
                let issue = Issue::issue(&ctx.stack, id).await?;
                if issue.status() == "open" {
                    open_count += 1
                } else if issue.status() == "close" {
                    close_count += 1
                }
                result.push(issue.json());
            }
        }
    }
    Ok(success(json!({"data": {
        "close_count": close_count,
        "open_count": open_count,
        "issues": result,
    }})))
}

/// # repository_issue_issue
/// close target issue
pub async fn repository_issue_close(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestIssueClose = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }
    let id = ObjectId::from_str(&data.issue_id)?;
    let issue = Issue::issue(&ctx.stack, id).await?;
    if issue.issue_type() != "main" {
        let err = format!("issue type error, current issue can't been closed");
        error!("{}", err);
        return Ok(failed(&err));
    }

    let new_issue = Issue::close(issue)?;
    put_object(&ctx.stack, &new_issue).await?;
    info!("issue close  put new issue object ok");

    let env = ctx.stack_env().await?;
    let object_id = new_issue.desc().calculate_id();
    let issue_map_key = issue_topic_key(&data.author_name, &data.name, &data.issue_id);
    let _ = env.set_with_path(&issue_map_key, &object_id, None, true).await?;
    let root = env.commit().await?;
    info!("issue close insert_with_key: {:?}", root);

    Ok(success(json!({
        "msg": "close issue ok"
    })))
}

/// # repository_issue_detail    
/// 仓库的issue 详情
pub async fn repository_issue_detail(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestIssueDetail = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    // let issue = RepositoryHelper::issue_by_path(stack, &data.author_name, &data.name, &data.issue_id, "0").await?;
    let mut comment_data: Vec<serde_json::Value> = Vec::new();

    let topic_id = data.issue_id;
    let topic = Issue::issue(&ctx.stack, ObjectId::from_str(&topic_id)?).await?;
    comment_data.push(topic.json());
    // let mut topic: Option<Issue> = None;

    // 获取 issue下面的评论回复

    let issue_topic_base = issue_comment_base(&data.author_name, &data.name, &topic_id);
    let env = ctx.stack_single_env().await?;
    if env.load_by_path(&issue_topic_base).await.is_ok() {
        loop {
            let ret = env.next(10).await?;
            if ret.len() == 0 {
                break;
            }

            for item in ret {
                println!("item: {:?}", item);
                let (_, id) = item.into_map_item();
                // let buf = get_object(&ctx.stack, id).await?;
                // let issue = Issue::clone_from_slice(&buf)?;

                let issue = Issue::issue(&ctx.stack, id).await?;
                // if issue.issue_type() == "main" {
                //     topic = Some(issue.clone())
                // }
                comment_data.push(issue.json());
            }
        }
    }
    // 按照创建时间排一下序
    comment_data.sort_by(sort_list_by_date);

    Ok(success(json!({
        "data": {
            "topic": topic.json(),
            "issues": comment_data,

        }
    })))
}

/// # repository_issue_comment   
/// 创建仓库的issue 回复(评论)
pub async fn repository_issue_comment(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestIssueComment = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }
    let topic_id = data.issue_id;
    // info!("issue comment: issue[{:?}], comment_id [{:?}]", &data.issue_id, comment_id);
    // 获取 用户名字
    let user_name = get_user_name_by_owner(&ctx.caller.to_string()).await?;
    info!("get user name[{}  {}] ", ctx.caller.to_string(), user_name);

    let issue = Issue::create(
        ctx.caller,
        "".to_string(),
        data.content,
        "".to_string(),
        "comment".to_string(),
        user_name,
        "0".to_string(),
    );
    put_object(&ctx.stack, &issue).await?;

    let env = ctx.stack_env().await?;
    let object_id = issue.desc().calculate_id();

    let issue_comment_key = issue_comment_key(
        &data.author_name,
        &data.name,
        &topic_id,
        &object_id.to_string(),
    );
    let r = env.insert_with_path(&issue_comment_key, &object_id).await?;
    info!("insert_with_key: {:?}", r);
    let _root = env.commit().await;

    Ok(success(json!({})))
}

/// # panel_issue_list    
/// 全局issue列表
pub async fn panel_issue_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    // let issues:Vec<Value> = CyfsGitDatabase::instance().await.unwrap()
    //         .select_panel_issue().await?;
    let env = ctx.stack_single_env().await?;
    let result = env.load_by_path(REPOSITORY_PATH).await;
    if result.is_err() {
        return Ok(success(json!({
            "data": {
                "issues": []
            }
        })));
    }

    let mut issues: Vec<serde_json::Value> = Vec::new();
    let mut data: Vec<(String, String)> = Vec::new();
    loop {
        let ret = env.next(10).await?;
        if ret.len() == 0 {
            break;
        }
        for item in ret {
            let (space, space_object_id) = item.into_map_item();
            let sub_env = ctx.stack_single_env().await?;
            sub_env.load(space_object_id).await?;

            loop {
                let sub_ret = sub_env.next(10).await.unwrap();
                if sub_ret.len() == 0 {
                    break;
                }
                for sub_item in sub_ret {
                    let (name, _) = sub_item.into_map_item();
                    data.push((space.clone(), name));
                }
            }
        }
    }

    for repo in data {
        let repo_full_name = issue_map_base_path(&repo.0, &repo.1);
        let env = ctx.stack_single_env().await?;
        if env.load_by_path(repo_full_name).await.is_ok() {
            loop {
                let ret = env.next(10).await?;
                if ret.len() == 0 {
                    break;
                }
                for item in ret {
                    let (_, id) = item.into_map_item();
                    let issue = Issue::issue(&ctx.stack, id).await;
                    if issue.is_err() {
                        continue;
                    }
                    let issue = issue.unwrap();
                    issues.push(json!({
                        "author_name": repo.0,
                        "name": repo.1,
                        "id": issue.id(),
                        "title": issue.title(),
                        "object_id": issue.id(),
                        "date": issue.date(),
                        "user_id": issue.owner(),
                        "content": issue.content(),
                        "status": issue.status(),
                        "user_name": issue.user_name(),
                    }));
                }
            }
        };
    }

    Ok(success(json!({
        "data": {
            "issues": issues
        }
    })))
}
