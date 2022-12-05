use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryInit {
    name: String,
    description: String,
    is_private: i32,
    author_type: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryNew {
    // owner: Option<String>,
    // author_id: String,
    name: String,
    description: String,
    is_private: i32,
    author_type: String,
    author_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryDelete {
    pub name: String,
    pub author_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryHome {
    pub name: String,
    pub author_name: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRepositoryList {
    #[serde(rename = "repo_name")]
    pub search_name: Option<String>,
    pub author_name: Option<String>,
}

type RequestRepositoryFind = RequestRepositoryDelete;
type RequestRepositorySetting = RequestRepositoryDelete;

/// # repository_init
/// init a pure rootstate repository
pub async fn repository_init(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryInit = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let people_id = ctx.caller.to_string();
    let repository = Repository::create(
        ctx.caller,
        data.name.clone(),
        data.description,
        data.is_private,
        data.author_type.clone(),
        people_id.clone(),
        0,
        "main".to_string(),
    );

    let _r = put_object(&ctx.stack, &repository).await?;
    let path_name = repository_object_map_path(&people_id, &repository.name());
    let env = ctx
        .stack
        .root_state_stub(None, Some(dec_id()))
        .create_path_op_env()
        .await?;
    let result = env.get_by_path(&path_name).await?;
    if result.is_some() {
        let msg = format!(
            "repository[{}/{}] was already created",
            repository.author_name(),
            repository.name()
        );
        error!("{}", msg);
        return Err(BuckyError::new(BuckyErrorCode::AlreadyExists, msg));
    }

    let _r = env
        .set_with_path(&path_name, &repository.desc().calculate_id(), None, true)
        .await?;
    let root = env.commit().await?;
    info!("add repository commit: {:?}", root);

    info!("repo full name {}/{}", ctx.caller, data.name);
    Ok(success(json!({"message": "ok"})))
}

/// # repository_new    
/// 新建仓库， object + git init
pub async fn repository_new(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryNew = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    // let space = data.author_name;
    // let name = data.name;
    // let (repository_key, _) = RepositoryHelper::object_map_path(&space, &name);
    // let env = stack.root_state_stub().create_path_op_env().await?;
    // // check key exist
    // let result = env.get_by_path(&repository_key).await?;
    // if result.is_some() {
    //     return Ok(failed(&format!("repository[{}] was already created", &name)))
    // }

    let repository = Repository::create(
        ctx.caller,
        data.name.clone(),
        data.description,
        data.is_private,
        data.author_type.clone(),
        data.author_name.clone(),
        0,
        "main".to_string(),
    );

    if repository.is_private() == 0 {
        info!(
            "send repository object to service {}  {}/{}",
            repository.desc().object_id(),
            repository.author_name(),
            repository.name()
        );
        post_special_object_service(&ctx.stack, &repository, "repository").await?;
    }
    let r = put_object(&ctx.stack, &repository).await?;
    info!("put_object local result: {:?}", r.result);

    // insert into sqlite
    insert_repository(&ctx.stack, &repository).await?;

    // 组织的仓库
    info!("repository space type: {}", data.author_type);
    if data.author_type == "org" {
        let env = ctx.stack_env().await?;
        let key = format!("{}/{}/{}", ORG_REPO_PATH, data.author_name, data.name);
        let _r = env
            .set_with_path(&key, &repository.desc().calculate_id(), None, true)
            .await?;
        let root = env.commit().await;
        info!("env set_with_path repository{:?}  result:{:?}", key, root);

        if ctx.is_other_caller() {
            // TO UP
            // let device = get_device_name(stack, &user_name).await?;
            let _ = put_object_target(
                &ctx.stack,
                &repository,
                Some(*ctx.source_device.object_id()),
                Some("repo/new".to_string()),
            )
            .await;
        }
    }

    // 本地data目录
    {
        let repo_target_dir = repository.repo_git_dir();
        std::fs::create_dir_all(&repo_target_dir)
            .map_err(|e| BuckyError::new(BuckyErrorCode::Failed, format!("{:?}", e)))?;
        info!("repo_target_dir: {:?}", &repo_target_dir);
        let _ = git_init(repo_target_dir.clone())?;
        git_config_quotepath(repo_target_dir);
    }

    Ok(success(json!({"message": "ok"})))
}

/// # repository_global_list    
/// pub仓库列表
pub async fn repository_global_list(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let (_, resp_body) = STACK_ACTION
        .get()
        .unwrap()
        .post_service("repo/list", &ctx.data)
        .await?;
    Ok(success_proxy(resp_body))
}

/// # repository_list    
/// 仓库列表
pub async fn repository_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryList = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if data.author_name.is_some() {
        let author_name = data.author_name.unwrap();
        if let Some(result) = ctx.check_space_proxy_request(&author_name).await? {
            return Ok(result);
        }
    }

    let env = ctx.stack_single_env().await?;
    let result = env.load_by_path(REPOSITORY_PATH).await;
    if result.is_err() {
        return Ok(success(json!({"data": []})));
    }

    let mut data: Vec<serde_json::Value> = Vec::new();
    let ret = env.list().await?;
    for item in ret {
        let (space, space_object_id) = item.into_map_item();
        let sub_env = ctx.stack_single_env().await?;
        sub_env.load(space_object_id).await?;
        let result = sub_env.list().await?;
        for sub_item in result {
            let (repository_name, _) = sub_item.into_map_item();
            info!("read repository [{}/{}] object", space, repository_name);
            let repository =
                RepositoryHelper::get_repository_object(&ctx.stack, &space, &repository_name).await;
            if let Ok(repository) = repository {
                data.push(repository.json());
            } else {
                error!("read repository object failed {:?}", repository.err());
            }
        }
    }

    //let mut data: Vec<serde_json::Value> = query_repository_from_sqlite(data.search_name).await?;

    // 远程ood访问的时候， filter is_private
    // if ctx.caller.eq(&get_owner(&ctx.stack).await) {
    //     data = data.iter().filter(|item| {
    //         if item["is_private"].as_i64() == Some(0) {
    //             true
    //         } else {
    //             false
    //         }
    //     }).cloned().collect();
    // }
    data.sort_by(sort_list_by_date_reverse);

    Ok(success(json!({ "data": data })))
}

/// # repository_find
/// 查找仓库 （用于 remote模块）
pub async fn repository_find(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryFind = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = ctx
        .repository_helper(space.clone(), name.clone())
        .repository()
        .await?;

    info!(
        "find [{}/{}], private: {}",
        space,
        name,
        repository.is_private()
    );
    if repository.is_private() == 1 && ctx.is_other_caller() {
        // check
        let member_list = crate::member_list(&ctx.stack, &space, &name).await?;
        if member_list.len() == 0 {
            return Ok(failed("target repo are not permission granted"));
        }

        let is_merber = member_list.iter().any(|member| {
            member["user_id"].as_str().unwrap().to_string() == ctx.caller.to_string()
        });

        if !is_merber {
            return Ok(failed("target repo are not permission granted"));
        }
    }

    Ok(success(json!({
        "repository": repository.json()
    })))
}

/// # repository_home    
/// 仓库首页信息
pub async fn repository_home(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let helper = ctx.repository_helper(space.clone(), name.clone());
    let repository = helper.repository().await?;
    let repository_value = repository.json();
    if repository.init() == 0 {
        return Ok(success(json!({
            "repository": repository_value,
            "is_setting": false,
            "star_count": 0,
            "branches": 0,
            "last_commit": {},
            "commit_count": 0,
        })));
    }
    let branches = repository.branches()?;

    let repo_dir = repository.repo_dir();
    let last_commit_info = git_log_last_commit_message(repo_dir.clone(), &data.branch)?;
    let commit_count = git_rev_list_count(repo_dir.clone(), &data.branch)?;
    // is setting
    let is_setting = ctx.caller.eq(&repository.desc().owner().unwrap());
    // 仓库的star数量
    let star_count = helper.start_count().await?;
    // TODO  other git infomation
    Ok(success(json!({
        "repository": repository_value,
        "is_setting": is_setting,
        "star_count": star_count,
        "branches": branches,
        "last_commit": last_commit_info,
        "commit_count": commit_count,
    })))
}

/// # repository_language_statistics    
/// 仓库的文件的语言统计
pub async fn repository_language_statistics(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    let repo_dir = repository.repo_dir();

    let hash = git_show_target_branch_ref(repo_dir.clone(), &data.branch)?;
    let cache_key = format!("language.stat.{}/{}/{}", space, name, hash);
    let languages = stat_repo_with_cache(repo_dir, &data.branch, &cache_key)?;

    Ok(success(json!({
        "data": languages,
    })))
}

/// # repository_setting_state_switch    
/// 切换仓库 的visible字段
pub async fn repository_setting_state_switch(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositorySetting = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    // 不用forward，只需要check
    let is_local = check_space_local(&ctx.stack, &space).await?;
    if !is_local {
        return Ok(failed("no permission"));
    }

    info!("to set repository visible");
    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    info!("current state {}", repository.is_private());

    let old_is_public = repository.is_private() == 0;
    let new_value = repository.is_private().clone() ^ 1;
    let new_repository = Repository::update(
        repository,
        &ctx.stack,
        json!({
            "target": "is_private",
            "value": new_value,
        }),
    )
    .await?;
    info!("update repository column visible success");
    // insert_repository(&ctx.stack, &repository).await?;

    if old_is_public {
        // public -> private
        let (_, value) = STACK_ACTION
            .get()
            .unwrap()
            .post_service(
                "repo/delete",
                &json!({"author_name": space, "name": name}).to_string(),
            )
            .await?;
        info!("set reposiotry public to private {}", value);
    } else {
        info!(
            "send repository object to service {}  {}/{}",
            new_repository.desc().object_id(),
            new_repository.author_name(),
            new_repository.name()
        );
        post_special_object_service(&ctx.stack, &new_repository, "repository").await?;
    }

    // update column in sqlite
    let db = CyfsGitDatabase::instance().await.unwrap();
    db.update_repository_visible(&space, &name, new_value)
        .await?;

    Ok(success(json!({})))
}

/// # repository_log_graph
/// log graph
pub async fn repository_log_graph(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryHome = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;

    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    let repo_dir = repository.repo_dir();

    let graph = git_log_graph(repo_dir, 0, 20)?;

    Ok(success(json!({
        "data": graph,
    })))
}

/// # repository_delete
/// 删除仓库
pub async fn repository_delete(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryDelete = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let env = ctx.stack_env().await?;

    let (repository_key, _) = RepositoryHelper::object_map_path(&space, &name);

    // check key exist
    let result = env.get_by_path(&repository_key).await?;
    if result.is_none() {
        return Ok(failed(&format!(
            "repository[{}/{}] not exist",
            &space, &name
        )));
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    // call dec-service delete
    if repository.is_private() == 0 {
        info!(
            "public repository call to service delete {}  {}/{}",
            repository.desc().object_id(),
            repository.author_name(),
            repository.name()
        );
        let _ = STACK_ACTION
            .get()
            .unwrap()
            .post_service("repo/delete", &ctx.data)
            .await
            .map_err(|e| {
                error!("post to dec service failed, {:?}", e);
                e
            })?;
    }

    sqlite_delete_repository(&space, &name).await?;

    // TODO check permission
    let object_id = result.unwrap();
    info!("result: {:?}", object_id);
    let _r = env
        .remove_with_path(&repository_key, Some(object_id))
        .await?;
    // 重复的去删除 创建，  commit可能会失败
    let root = env.commit().await;
    println!("remove commit: {:?}", root);

    std::fs::remove_dir_all(RepositoryHelper::repo_dir(&space, &name)).map_err(|e| {
        BuckyError::new(BuckyErrorCode::InvalidParam, format!("删除仓库失败{:?}", e))
    })?;

    delete_object(&ctx.stack, object_id).await?;
    Ok(success(json!({"message": "ok"})))
}

/// # remote_repository_delete
/// 删除仓库(仅map)
pub async fn remote_repository_delete(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryDelete = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let env = ctx.stack_env().await?;

    let (repository_key, _) = RepositoryHelper::object_map_path(&space, &name);

    // check key exist
    let result = env.get_by_path(&repository_key).await?;
    if result.is_none() {
        return Ok(failed(&format!(
            "repository[{}/{}] not exist",
            &space, &name
        )));
    }

    let _r = env.remove_with_path(&repository_key, None).await?;
    let root = env.commit().await;
    println!("remove commit: {:?}", root);

    Ok(success(json!({"message": "ok"})))
}
