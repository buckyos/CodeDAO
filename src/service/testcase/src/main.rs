use std::str::FromStr;

use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_debug::*;
use cyfs_lib::*;
// use std::str::FromStr;
use clap::Parser;
use cyfs_git_base::*;
use log::*;
// use sqlx::*;

mod api;
use crate::api::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(short, long)]
    email: Option<String>,

    #[clap(short, long)]
    file: Option<String>,

    #[clap(short, long)]
    service: Option<String>,

    // request data
    #[clap(short, long)]
    data: Option<String>,
}

pub async fn get_owner(stack: &Arc<SharedCyfsStack>) -> ObjectId {
    let device = stack.local_device();
    let owner = device
        .desc()
        .owner()
        .to_owned()
        .unwrap_or_else(|| device.desc().calculate_id());
    owner
}

pub async fn test_db(stack: &Arc<SharedCyfsStack>) -> BuckyResult<()> {
    info!("init db");
    let result = CyfsGitDatabase::init().await;
    if result.is_err() {
        println!("err, {}", result.err().unwrap());
        return Ok(());
    }
    let db = CyfsGitDatabase::instance().await.unwrap();
    let _ = db.select_user().await;
    info!("select user ok");
    // let mut conn = db.get_conn().await?;

    // let sql = "drop table organization";
    // let r = sqlx::query(sql).execute(&mut *conn).await.unwrap();
    // info!("drop table ok {:?}", r);
    // db.init_organization_table();

    // db.init_tables().await;
    // db.init_issue_topic_table().await.unwrap();
    //

    //let r = db.count_issue_id("sunxinle", "0309").await?;
    // info!("count_issue_id db ", r);
    // info!("count_issue_id db ", r);

    // sync_users(stack, db).await;
    // info!("sync user ok");

    // db.select_user().await;
    // info!("select user ok");

    // let result = get_owner_id_by_name(db, "sunxinle").await.unwrap();
    // info!("get_owner_id_by_name ok {}", result);

    Ok(())
}

pub async fn test_cache() -> BuckyResult<()> {
    CyfsGitCache::new();
    CyfsGitCache::put("test", "ok");

    info!("cache ok {:?}", CyfsGitCache::get("test").unwrap());

    Ok(())
}

pub async fn test_remote_some_key(stack: &Arc<SharedCyfsStack>) -> BuckyResult<()> {
    let env = stack
        .root_state_stub(None, Some(dec_id()))
        .create_path_op_env()
        .await?;
    // let topic_base_key = RepositoryHelper::issue_base_path("sunxinle", "0309");
    let group_key = format!("{}/{}", ORG_LIST_PATH, "group01");
    let _r = env.remove_with_path(&group_key, None).await?;
    let root = env.commit().await;

    info!("remove_with_path ok {:?}", root);

    Ok(())
}

#[async_std::main]
async fn main() -> BuckyResult<()> {
    CyfsLoggerBuilder::new_app("cyfs-git-test")
        .level("info")
        .console("info")
        .enable_bdt(Some("error"), Some("error"))
        .build()
        .unwrap()
        .start();

    ConfigManager::new_oncecell_with_content(
        r#"
    [main]
    channel="dev-test"
    deploy_owner_id="5r4MYfFQz9iEzjwHUpc79CwrvXqsh7xUzynkiTUEckxB"
    public_service_ood="5aSixgM1oBicrsUdS3nyKM1MA9AgiMEE2y2qFQ3jTTYB""#,
    );

    let dec_id = "9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r";
    let dec_id = ObjectId::from_str(dec_id)?;
    let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id)).await.unwrap());
    stack.wait_online(None).await.unwrap();
    let owner = get_owner(&stack).await;

    let _ = STACK_ACTION.set(StackActionStruct {
        stack: stack.clone(),
        owner: owner,
        dec_id: dec_id,
    });

    let cli = Cli::parse();
    if cli.name.is_some() {
        let name = cli.name.unwrap();
        let email = cli.email;
        let file = cli.file;

        let api = Api::new(Arc::clone(&stack), name.as_str(), cli.data);

        match name.as_str() {
            "decid"=> test_dec_id(&stack).await,
            "debug" => debug_info(&stack, name.as_str()).await,
            "user/init" => api.user_init().await,
            "user/checkInit" => user_check_init(&stack).await,
            "user/setting" => user_setting(&stack, email).await,
            "user/list" => user_list(&stack).await,
            "user/getByName" => user_get_by_name(&stack).await,
            "organization/new" => organization_new(&stack).await,
            "organization/list" => organization_list(&stack).await,
            "repo/new" => repository_new(&stack, name.as_str()).await,
            "repo/new/private" => repository_private_new(&stack).await,
            "repo/list" => repository_list(&stack, name.as_str()).await,
            "repo/log/graph" => repository_log_graph(&stack, name.as_str()).await,
            "repo/global/list" => api.common().await,	    
            "repo/delete" => repository_delete(&stack, name.as_str()).await,
            "repo/push/head" => repository_push_head(&stack, name.as_str()).await,
            "repo/file" => repository_file(&stack, name.as_str(), file).await,
            "repo/home" => repository_home(&stack, name.as_str()).await,
            "repo/commits" => repository_commits(&stack, name.as_str()).await,
            "repo/commit" => repository_commit(&stack, name.as_str()).await,
            "repo/member" => api.common().await,
            "repo/member/add" => api.common().await,
            "issue/new" => repository_issue_new(&stack, name.as_str()).await,
            "repo/issues" => repository_issue_list(&stack, name.as_str()).await,
            _ => api.user_init().await,
        };
    }

    Ok(())
}
