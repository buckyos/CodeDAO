use cyfs_lib::*;
use cyfs_base::*;
use log::*;
use async_std::sync::Arc;
use sqlx::*;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
use crate::*;


pub async fn sync_users(stack: &Arc<SharedCyfsStack>, db: &CyfsGitDatabase) -> BuckyResult<()> {
    let env = stack.root_state_stub(None, Some(dec_id())).create_single_op_env().await?;
    env.load_by_path(USER_LIST_PATH).await?;

    loop {
        let ret = env.next(10).await.unwrap();
        if ret.len() == 0 {
            break;
        }

        for item in ret {
            let (_,id) = item.into_map_item();
            let buf = get_object(stack, id).await?;
            let user = UserInfo::clone_from_slice(&buf)? as UserInfo;
            let owner_id = user.desc().owner().unwrap().to_string();
            info!("user name insert into db, {}", user.name());
            let _ = db.insert_user(&user.id(), user.name(), &owner_id).await;
        }
    }

    Ok(())
}


pub async fn get_owner_id_by_name(db: &CyfsGitDatabase, name: &str) -> BuckyResult<String> {
    let row = db.fetch_user_by_name(name).await?;
    let owner_id: String = row.get("owner_id");
    Ok(owner_id)
}

pub async fn get_name_by_user_id(db: &CyfsGitDatabase, user_id: &str) -> BuckyResult<String> {
    let row = db.fetch_user_by_id(user_id).await?;
    let name: String = row.get("name");
    Ok(name)
}


/// sqlite_delete_user_info
pub async fn sqlite_delete_user_info(owner_id: &str) -> BuckyResult<()> {
    info!("delete user[{}] info ", owner_id);
    let sql = "DELETE FROM users WHERE owner_id = ?1";
    let db = CyfsGitDatabase::instance().await.unwrap();
    let mut conn = db.get_conn().await?;
    let _r = sqlx::query(sql)
        .bind(owner_id)
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            let err_msg = format!("delete users[{}] failed {:?}", owner_id, e);
            error!("{}", err_msg);
            BuckyError::new(BuckyErrorCode::SqliteError, err_msg)
        })?;
    Ok(())
}