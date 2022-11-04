// use cyfs_lib::*;
use cyfs_base::*;
// use std::str::FromStr;
use serde_json::json;
use async_std::sync::{Mutex, MutexGuard};
use sqlx::*;
use sqlx::sqlite::*;
// use sqlx::query::{Query, QueryAs};
use log::*;
use once_cell::sync::OnceCell;
use crate::*;


macro_rules! db_err {
    ($($arg:tt)*) => (|err: sqlx::Error | {
        error!("{} {:?}", $($arg)*, err);
        BuckyError::new(BuckyErrorCode::SqliteError, format!("{:?} {}", $($arg)?, err))
    })
}

pub struct CyfsGitDatabase {
    conn: Mutex<SqliteConnection>,
}

pub static DATABASE: OnceCell<CyfsGitDatabase> = OnceCell::new();

impl  CyfsGitDatabase{
    pub async fn init() -> BuckyResult<()> {
        let conn = CyfsGitDatabase::conn().await?;
        let db = CyfsGitDatabase{
            conn: Mutex::new(conn)
        };
        let _ = DATABASE.set(db);
        
        // Ok(CyfsGitDatabase{
        //     conn: Mutex::new(conn)
        // })
        Ok(())
    }

    pub async fn instance() -> BuckyResult<&'static CyfsGitDatabase> {
        let db = DATABASE.get();
        if db.is_none() {
            return Err(BuckyError::new(BuckyErrorCode::ErrorState, "empty db instance"))
        }
        let db = db.unwrap();
        Ok(db)
    }

    pub async fn conn() -> BuckyResult<SqliteConnection> {
        let mut db_path = cyfs_util::get_cyfs_root_path();
        db_path.push("data");
        db_path.push("app");
        db_path.push(dec_id().to_string());
        db_path.push(CYFS_GIT_DB_BASE_PATH);
        // let db_path = cyfs_util::get_app_data_dir(CYFS_GIT_DB_BASE_PATH);
        if !db_path.exists() {
            std::fs::create_dir_all(&db_path).map_err(|e| {
                BuckyError::new(BuckyErrorCode::Failed, format!("{:?}", e))
            })?;
        }

        let db_path= db_path.join(format!("cyfs-git-{}.db", dec_id()));
        info!("init sqlite db path {:?}", db_path);
        
        let options = SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true);
    
        let conn = options.connect().await.map_err(db_err!("sqliteconnect failed"))?;
        // self.conn = Mutex::new(conn);
        Ok(conn)
    }

    // init_tables_service
    // 初始化 table in service)
    pub async fn init_tables_service(&self) -> BuckyResult<()>  {
        // 创建 users table
        self.init_user_table().await?;
        // 创建 organizations table
        self.init_organization_table().await?;
        // 创建 repository
        self.init_repository_table().await?;
        Ok(())
    }

    // init_tables_app
    // 初始化 table in app
    pub async fn init_tables_app(&self) -> BuckyResult<()>  {
        // 创建 repository
        self.init_repository_table().await?;
        // self.init_issue_topic_table().await?;
        Ok(())
    }
    
    // init user table
    pub async fn init_user_table(&self) -> BuckyResult<()>  {
        let mut conn = self.get_conn().await?;
        let result = sqlx::query("select 1 from users limit 1").fetch_one(&mut *conn).await;
        if result.is_ok() {
            info!("users table is exist, no need to create");
            return Ok(())
        }
        let sql = r#"CREATE TABLE IF NOT EXISTS "users" (
            "object_id" CHAR(45) PRIMARY KEY NOT NULL UNIQUE,
            "name" CHAR(100) NOT NULL UNIQUE,
            "owner_id" CHAR(100) NOT NULL UNIQUE
        )"#;
        let r = conn.execute(sqlx::query(sql)).await.map_err(db_err!("create user tables failed"))?;
        info!("create tables users ok {:?}", r);
        Ok(())
    }

    pub async fn init_organization_table(&self) -> BuckyResult<()>  {
        let mut conn = self.get_conn().await?;
        let result = sqlx::query("select 1 from organization limit 1").fetch_one(&mut *conn).await;
        if result.is_ok() {
            info!("organization table is exist, no need to create");
            return Ok(())
        }
        info!("organization table not exist, start to create table and index");
        let sql = r#"
        CREATE TABLE IF NOT EXISTS "organization" (
            "id" INTEGER PRIMARY KEY autoincrement,
            "name" CHAR(100) NOT NULL UNIQUE,
            "org_id" CHAR(100) NOT NULL,
            "email" CHAR(100) NOT NULL,
            "description" CHAR(100) NOT NULL,
            "creator" CHAR(100) NOT NULL,
            "avatar" CHAR(100) NOT NULL
        );
        "#;
        let r = conn
            .execute(sqlx::query(sql)).await
            .map_err(db_err!("create tables organization failed"))?;
        info!("create tables organization ok {:?}", r);
        Ok(())
    }

    /// init_repository_table
    /// init repository table
    pub async fn init_repository_table(&self) -> BuckyResult<()>  {
        let mut conn = self.get_conn().await?;
        let result = sqlx::query("select 1 from repository limit 1").fetch_one(&mut *conn).await;
        if result.is_ok() {
            info!("repository table is exist, no need to create");
            return Ok(())
        }

        info!("repository table not exist, start to create table and index");
        let sql = r#"
        CREATE TABLE IF NOT EXISTS "repository" (
            "id" INTEGER PRIMARY KEY autoincrement,
            "name" CHAR(100) NOT NULL UNIQUE,
            "description" CHAR(100) NOT NULL,
            "init" INTEGER NOT NULL,
            "is_private" INTEGER NOT NULL,
            "fork_from_id" CHAR(100) NOT NULL,
            "author_type" CHAR(100) NOT NULL,
            "author_name" CHAR(100) NOT NULL,
            "created_at" INTEGER NOT NULL
        );
        "#;
        let r = conn
            .execute(sqlx::query(sql)).await
            .map_err(db_err!("create tables repository failed"))?;
        info!("create tables repository ok {:?}", r);

        // TODO unique key: author_name and name
        Ok(())
    }



    // init_issue_topic_table
    // 初始化表
    pub async fn init_issue_topic_table(&self) -> BuckyResult<()>  {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query("select 1 from issue_topics limit 1").fetch_one(&mut *conn).await;

        if !result.is_ok() {
            info!("issue_topics table not exist, start to create table");
            // 创建 users table
            let sql = r#"
            CREATE TABLE IF NOT EXISTS "issue_topics" (
                "id" INTEGER PRIMARY KEY autoincrement,
                "author_name" CHAR(100) NOT NULL,
                "name" CHAR(100) NOT NULL,
                "user_name" CHAR(100) NOT NULL,
                "id_in_repo" CHAR(100) NOT NULL,
                "title" CHAR(100) NOT NULL,
                "content" text,
                "status" CHAR(100) NOT NULL,
                "issue_type" CHAR(100) NOT NULL,
                "created_at" INTEGER NOT NULL
            );"#;
            let r = sqlx::query(sql).execute(&mut *conn).await.map_err(db_err!("create tables issue_topics failed"))?;
            info!("create tables users ok {:?}", r);
        }

        // 创建索引
        let result = sqlx::query("SELECT * FROM sqlite_master  WHERE type='index' and name='author_name_idx' and tbl_name='issue_topics'").fetch_one(&mut *conn).await;
        if !result.is_ok() {
            let r = conn
                .execute(sqlx::query("CREATE INDEX author_name_idx ON issue_topics (author_name);")).await
                .map_err(db_err!("create index author_name_idx failed"))?;
            info!("create index author_name_idx on issue_topics ok {:?}", r);
        }

        let result = sqlx::query("SELECT * FROM sqlite_master  WHERE type='index' and name='name_idx' and tbl_name='issue_topics'").fetch_one(&mut *conn).await;
        if !result.is_ok() {
            let r = conn.execute(sqlx::query("CREATE INDEX name_idx ON issue_topics (name);")).await.map_err(db_err!("create index name_idx failed"))?;
            info!("create index name_idx on issue_topics ok {:?}", r);
        }

        info!("init issue_topics tables ok and index ok");
        Ok(())
    }

    pub async fn get_conn(&self) -> BuckyResult<MutexGuard<'_, SqliteConnection>> {
        Ok(self.conn.lock().await)
    }

    pub async fn count_issue_id(&self, author_name: &str, name: &str) -> BuckyResult<i32> {
        let sql = "select count(*) from issue_topics where author_name=?1 and name=?2";
        let mut conn = self.get_conn().await?;

        info!("sqlx count issue_topics into db, {}, {}", author_name, name);
        let r = sqlx::query(sql)
            .bind(author_name)
            .bind(name)
            .fetch_one(&mut *conn)
            .await.map_err(db_err!("count issue_topics failed {:?}"))?;
        let count:i32 = r.get("count(*)");
        // info!("count issue_topics ok {:?}", r);
        info!("count issue_topics ok {:?}", count);

        Ok(count + 1)
    }

    pub async fn insert_issue_topic(&self, 
        author_name: &str, 
        name: &str, 
        user_name: &str, 
        id_in_repo:i32,
        title: &str, 
        content: &str, 
        status: &str, 
        issue_type: &str, 
    ) -> BuckyResult<()> {
        let sql = "insert into issue_topics (author_name, name, user_name,id_in_repo,title,content,status,issue_type) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
        let mut conn = self.get_conn().await?;

        info!("sqlx execute insert into db, {}", name);
        let r = sqlx::query(sql)
            .bind(author_name)
            .bind(name)
            .bind(user_name)
            .bind(id_in_repo)
            .bind(title)
            .bind(content)
            .bind(status)
            .bind(issue_type)
            .execute(&mut *conn)
            .await.map_err(db_err!("insert users failed {:?}"))?;
        info!("insert users ok {:?}", r);
        Ok(())
    }

    pub async fn select_issue_topic(&self, author_name: &str, name: &str) -> BuckyResult<Vec<serde_json::Value>> {
        let sql = "select * from issue_topics where author_name=?1 and name=?2";
        let mut conn = self.get_conn().await?;
        let result:Vec<SqliteRow> = sqlx::query(sql)
            .bind(author_name)
            .bind(name)
            .fetch_all(&mut *conn).await.map_err(db_err!("select issue_topics failed {:?}"))?;

        let mut response_data: Vec<serde_json::Value> = Vec::new();
        for issue_topic in result {
            response_data.push(json!({
                "id": issue_topic.get::<&str, &str>("id_in_repo"),
                "title": issue_topic.get::<&str, &str>("title"),
                "user_name": issue_topic.get::<&str, &str>("user_name"),
            }))
        }

        // let mut response_data: Vec<serde_json::Value> = Vec::new();
        Ok(response_data)
    }

    pub async fn select_panel_issue(&self) -> BuckyResult<Vec<serde_json::Value>> {
        let sql = "select * from issue_topics";
        let mut conn = self.get_conn().await?;
        let result:Vec<SqliteRow> = sqlx::query(sql)
            .fetch_all(&mut *conn).await.map_err(db_err!("select issue_topics failed {:?}"))?;

        let mut response_data: Vec<serde_json::Value> = Vec::new();
        for issue_topic in result {
            response_data.push(json!({
                "id": issue_topic.get::<&str, &str>("id_in_repo"),
                "title": issue_topic.get::<&str, &str>("title"),
                "user_name": issue_topic.get::<&str, &str>("user_name"),
                "author_name": issue_topic.get::<&str, &str>("author_name"),
                "name": issue_topic.get::<&str, &str>("name"),
            }))
        }
        // let mut response_data: Vec<serde_json::Value> = Vec::new();
        Ok(response_data)
    }

    pub async fn insert_user(&self, id: &str, name: &str, owner_id: &str) -> BuckyResult<()> {
        let sql = "insert into users (object_id, name, owner_id) values (?1, ?2, ?3)";
        let mut conn = self.get_conn().await?;

        info!("sqlx execute insert into db, {}", name);
        let r = sqlx::query(sql)
            .bind(id)
            .bind(name)
            .bind(owner_id)
            .execute(&mut *conn)
            .await.map_err(db_err!("insert users failed {:?}"))?;
        info!("insert users ok {:?}", r);
        Ok(())
    }

    // insert_repository
    pub async fn insert_repository(
        &self,
        name: &str,
        description: &str,
        init: i32,
        is_private: i32,
        fork_from_id: &str,
        author_type: &str,
        author_name: &str,
        created_at: i64,
    ) -> BuckyResult<()> {
        let sql = "insert into repository (name,description,init,is_private, fork_from_id,author_type, author_name, created_at) values (?1,?2,?3,?4,?5,?6,?7,?8)";
        let mut conn = self.get_conn().await?;
        info!("sqlx execute insert repository data into db, {} {} {}", author_type, author_name, name);
        let r = sqlx::query(sql)
            .bind(name)
            .bind(description)
            .bind(init)
            .bind(is_private)
            .bind(fork_from_id)
            .bind(author_type)
            .bind(author_name)
            .bind(created_at)
            .execute(&mut *conn)
            .await.map_err(db_err!("insert repository failed {:?}"))?;
        info!("insert repository ok {:?}", r);
        Ok(())
    }

    // 更改 repository 的is_private字段
    pub async fn update_repository_visible(
        &self,
        author_name: &str,
        name: &str,
        is_private: i32,
    ) -> BuckyResult<()> {
        let sql = "UPDATE repository SET is_private = ?1 WHERE author_name=?2 AND name=?3";
        let mut conn = self.get_conn().await?;
        let _r = sqlx::query(sql)
            .bind(is_private)
            .bind(author_name)
            .bind(name)
            .execute(&mut *conn)
            .await.map_err(db_err!("UPDATE repository is_private failed {:?}"))?;
        Ok(())
    }



    // insert_organization
    // 添加org
    pub async fn insert_organization(&self, name: &str, email: &str, description: &str, creator: &str, avatar: &str, org_id: &str) -> BuckyResult<()> {
        let sql = "insert into organization (name,email,description,creator,avatar, org_id) values (?1,?2,?3,?4,?5,?6)";
        let mut conn = self.get_conn().await?;

        info!("sqlx execute insert into db, {}", name);
        let r = sqlx::query(sql)
            .bind(name)
            .bind(email)
            .bind(description)
            .bind(creator)
            .bind(avatar)
            .bind(org_id)
            .execute(&mut *conn)
            .await.map_err(db_err!("insert organization failed {:?}"))?;
        info!("insert organization ok {:?}", r);
        Ok(())
    }

    pub async fn fetch_repository(&self, author_name: &str, name: &str) -> BuckyResult<SqliteRow> {
        let sql = "select * from repository where author_name=?1 and name=?2";
        let mut conn = self.get_conn().await?;
        let r:SqliteRow = sqlx::query(sql)
            .bind(author_name)
            .bind(name)
            .fetch_one(&mut *conn)
            .await.map_err(db_err!("select repository failed {:?}"))?;
        Ok(r)
    }



    pub async fn select_user(&self) -> BuckyResult<()> {
        let sql = "select * from users";
        let mut conn = self.get_conn().await?;
        let r = sqlx::query(sql).fetch_all(&mut *conn).await.map_err(db_err!("select users failed {:?}"))?;
        for user in r {
            let name: String = user.get("name");
            info!("fetch_all, {}", name);
        }
        Ok(())
    }

    pub async fn fetch_user_by_name(&self, name: &str) -> BuckyResult<SqliteRow> {
        let sql = "select * from users where name=?1";
        let mut conn = self.get_conn().await?;
        let r:SqliteRow = sqlx::query(sql)
            .bind(name)
            .fetch_one(&mut *conn)
            .await.map_err(db_err!("select target user failed {:?}"))?;
        Ok(r)
    }

    pub async fn fetch_user_by_id(&self, owner_id: &str) -> BuckyResult<SqliteRow> {
        let sql = "select * from users where owner_id=?1";
        let mut conn = self.get_conn().await?;
        let r:SqliteRow = sqlx::query(sql)
            .bind(owner_id)
            .fetch_one(&mut *conn)
            .await.map_err(db_err!("select target user failed {:?}"))?;
        Ok(r)
    }

    pub async fn fetch_org_by_name(&self, name: &str) -> BuckyResult<OrganizationData> {
        let sql = "select * from organization where name=?1";
        let mut conn = self.get_conn().await?;
        let r:SqliteRow = sqlx::query(sql)
            .bind(name)
            .fetch_one(&mut *conn)
            .await.map_err(db_err!("select organization failed {:?}"))?;
        Ok(OrganizationData{
            name: r.get("name"),
            email: r.get("email"),
            avatar: r.get("avatar"),
            creator: r.get("creator"),
            description: r.get("description"),
            org_id: r.get("org_id"),
        })
    }

    pub async fn exec(&self) -> BuckyResult<()>  {
        Ok(())
    }
}
