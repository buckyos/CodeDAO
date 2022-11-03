use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_lib::*;
use log::*;
use sqlx::sqlite::*;
use sqlx::*;

// use serde::{Deserialize, Serialize};
use crate::*;
use serde_json::{json, Value};

pub async fn insert_repository(
    stack: &Arc<SharedCyfsStack>,
    repository: &Repository,
) -> BuckyResult<()> {
    let (repository_key, _) =
        RepositoryHelper::object_map_path(&repository.author_name(), &repository.name());
    let env = stack
        .root_state_stub(None, Some(dec_id()))
        .create_path_op_env()
        .await?;
    // check key exist
    info!("add publish repository {}", repository_key);
    let result = env.get_by_path(&repository_key).await?;
    if result.is_some() {
        error!("repository[{}] was already created", repository_key);
        return Err(BuckyError::new(
            BuckyErrorCode::AlreadyExists,
            format!("repository[{}] was already created", &repository_key),
        ));
    }

    let object_id = repository.desc().calculate_id();

    let r = env
        .set_with_path(&repository_key, &object_id, None, true)
        .await?;
    info!(
        "env set_with_path repository{:?}  result:{:?}",
        repository_key, r
    );

    let root = env.commit().await;
    info!("add repository commit: {:?}", root);

    // insert to sqlite
    let db = CyfsGitDatabase::instance().await.unwrap();
    insert_repository_to_sqlite(repository, db).await?;

    Ok(())
}

/// query_repository_from_sqlite
pub async fn query_repository_from_sqlite(name: Option<String>) -> BuckyResult<Vec<Value>> {
    let mut data: Vec<Value> = Vec::new();

    let db = CyfsGitDatabase::instance().await.unwrap();
    let mut conn = db.get_conn().await?;

    let result: Vec<SqliteRow> = if name.is_some() {
        let sql = format!(
            "select * from repository where name like '%{}%'",
            name.unwrap()
        );
        sqlx::query(&sql)
            // .bind(name.unwrap())
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("select issue_topics failed {:?}", e);
                BuckyError::new(BuckyErrorCode::SqliteError, format!("{:?}", e))
            })?
    } else {
        let sql = "select * from repository";
        sqlx::query(sql).fetch_all(&mut *conn).await.map_err(|e| {
            error!("select issue_topics failed {:?}", e);
            BuckyError::new(BuckyErrorCode::SqliteError, format!("{:?}", e))
        })?
    };
    for repository in result {
        data.push(json!({
            "name": repository.get::<&str, &str>("name"),
            "author_name": repository.get::<&str, &str>("author_name"),
            "is_private": repository.get::<i32, &str>("is_private"),
            "date": repository.get::<i64, &str>("created_at"),
        }))
    }

    Ok(data)
}

/// query_repository
/// query in object map
pub async fn query_repository(stack: &Arc<SharedCyfsStack>) -> BuckyResult<Vec<Repository>> {
    let mut data: Vec<Repository> = Vec::new();
    let env = stack
        .root_state_stub(None, Some(dec_id()))
        .create_single_op_env()
        .await?;
    let result = env.load_by_path(REPOSITORY_PATH).await;
    if result.is_err() {
        return Ok(data);
    }

    loop {
        let ret = env.next(10).await?;
        if ret.len() == 0 {
            break;
        }
        for item in ret {
            // println!("item: {:?}", item);
            let (space, space_object_id) = item.into_map_item();
            // env.load_by_path(format!("{}{}",REPOSITORY_PATH, space)).await

            let sub_env = stack
                .root_state_stub(None, Some(dec_id()))
                .create_single_op_env()
                .await?;
            sub_env.load(space_object_id).await?;

            loop {
                let sub_ret = sub_env.next(10).await.unwrap();
                if sub_ret.len() == 0 {
                    break;
                }
                for sub_item in sub_ret {
                    trace!("query repository sub_item: {:?}", sub_item);
                    let (name, _) = sub_item.into_map_item();
                    let result =
                        RepositoryHelper::get_repository_object(stack, &space, &name).await;
                    if result.is_ok() {
                        let repository = result.unwrap();
                        data.push(repository);
                    }
                }
            }
        }
    }
    Ok(data)
}

/// sync_repository_data
pub async fn sync_repository_data(
    stack: &Arc<SharedCyfsStack>,
    db: &CyfsGitDatabase,
) -> BuckyResult<()> {
    let data: Vec<Repository> = query_repository(stack).await?;

    for repository in data {
        insert_repository_to_sqlite(&repository, db).await?;
    }
    info!("sync all repository data into sqlite ok");

    Ok(())
}

/// sync_repository_data
pub async fn insert_repository_to_sqlite(
    repository: &Repository,
    db: &CyfsGitDatabase,
) -> BuckyResult<()> {
    let author_name = repository.author_name();
    let name = repository.name();
    trace!(
        "trying to insert current repo[{}/{}] to sqlite",
        author_name,
        name
    );
    if let Ok(_) = db.fetch_repository(author_name, name).await {
        trace!("query data exist in sqlite [ {}/{} ]", author_name, name);
    } else {
        db.insert_repository(
            name,
            repository.description(),
            repository.init(),
            repository.is_private(),
            repository.fork_from_id(),
            repository.author_type(),
            author_name,
            repository.date() as i64,
        )
        .await?;
        info!(
            "sync repository[{}/{}] data to sqlite ok",
            author_name, name
        );
    }
    Ok(())
}

/// sqlite_delete_repository
pub async fn sqlite_delete_repository(author_name: &str, name: &str) -> BuckyResult<()> {
    // info!("trying to insert current repo[{}/{}] to sqlite", author_name, name);
    let sql = "DELETE FROM repository WHERE id = (SELECT id FROM repository WHERE author_name=?1 and name=?2  LIMIT 1 )";
    let db = CyfsGitDatabase::instance().await.unwrap();
    let mut conn = db.get_conn().await?;

    let _r = sqlx::query(sql)
        .bind(author_name)
        .bind(name)
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            let err_msg = format!("DELETE repository failed {:?}", e);
            error!("{}", err_msg);
            BuckyError::new(BuckyErrorCode::SqliteError, err_msg)
        })?;

    info!("delete result");
    Ok(())
}
