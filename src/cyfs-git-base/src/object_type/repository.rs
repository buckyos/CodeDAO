use async_std::sync::Arc;
use async_trait::async_trait;
use cyfs_base::*;
use cyfs_lib::*;
use log::*;
use serde_json::{json, Value};
use std::path::PathBuf;
// use super::issue::*;
use crate::*;

// #[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::RepositoryDescContent)]
pub struct RepositoryDescContent {
    name: String,
    description: String,
    init: i32,
    is_private: i32,
    fork_from_id: String,
    author_type: String,
    author_name: String,
}

impl DescContent for RepositoryDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Repository as u16
    }
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
    type OwnerType = Option<ObjectId>;
    type AreaType = SubDescNone;
    type AuthorType = SubDescNone;
    type PublicKeyType = SubDescNone;
}

#[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
pub struct RepositoryBodyContent {}

impl BodyContent for RepositoryBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type RepositoryType = NamedObjType<RepositoryDescContent, RepositoryBodyContent>;
type RepositoryBuilder = NamedObjectBuilder<RepositoryDescContent, RepositoryBodyContent>;
// type RepositoryDesc = NamedObjectDesc<RepositoryDescContent>;

pub type RepositoryId = NamedObjectId<RepositoryType>;
pub type Repository = NamedObjectBase<RepositoryType>;

pub trait RepositoryObject {
    fn create(
        owner: ObjectId,
        name: String,
        description: String,
        is_private: i32,
        author_type: String,
        author_name: String,
        init: i32,
    ) -> Self;

    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn name(&self) -> &String;
    fn description(&self) -> &String;
    fn init(&self) -> i32;
    fn is_private(&self) -> i32;
    fn author_type(&self) -> &String;
    fn author_name(&self) -> &String;
    fn fork_from_id(&self) -> &String;
}

impl RepositoryObject for Repository {
    fn create(
        owner: ObjectId,
        name: String,
        description: String,
        is_private: i32,
        author_type: String,
        author_name: String,
        init: i32,
    ) -> Self {
        let desc = RepositoryDescContent {
            name,
            description,
            init,
            is_private,
            fork_from_id: "".to_string(),
            author_type,
            author_name,
        };
        let body = RepositoryBodyContent {};

        RepositoryBuilder::new(desc, body)
            .owner(owner)
            .dec_id(dec_id())
            .option_create_time(None)
            .build()
    }
    fn id(&self) -> String {
        self.desc().calculate_id().to_string()
    }
    fn date(&self) -> u64 {
        bucky_time_to_js_time(self.desc().create_time())
    }
    fn init(&self) -> i32 {
        self.desc().content().init
    }
    fn name(&self) -> &String {
        &self.desc().content().name
    }
    fn description(&self) -> &String {
        &self.desc().content().description
    }
    fn is_private(&self) -> i32 {
        self.desc().content().is_private
    }
    fn author_type(&self) -> &String {
        &self.desc().content().author_type
    }
    fn author_name(&self) -> &String {
        &self.desc().content().author_name
    }
    fn fork_from_id(&self) -> &String {
        &self.desc().content().fork_from_id
    }
}

#[async_trait]
pub trait RepositoryObjectUtil {
    fn update_column(repository: Repository, column: Value) -> BuckyResult<Repository>;
    async fn update(
        repository: Repository,
        stack: &Arc<SharedCyfsStack>,
        column: Value,
    ) -> BuckyResult<Repository>;

    async fn repository(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<Repository>;
    fn repo_dir(&self) -> PathBuf;
    fn repo_git_dir(&self) -> PathBuf;
    fn branches(&self) -> BuckyResult<Vec<String>>;
    fn json(&self) -> Value;
}

#[async_trait]
impl RepositoryObjectUtil for Repository {
    // update_ colunm
    fn update_column(repository: Repository, column: Value) -> BuckyResult<Repository> {
        let mut desc = RepositoryDescContent {
            name: repository.name().to_string(),
            description: repository.description().to_string(),
            init: repository.init(),
            is_private: repository.is_private(),
            fork_from_id: repository.fork_from_id().to_string(),
            author_type: repository.author_type().to_string(),
            author_name: repository.author_name().to_string(),
        };

        let target_column = column["target"].as_str();
        if target_column.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::InvalidParam,
                "empty target key",
            ));
        }
        let target_column = target_column.unwrap();
        match target_column {
            "is_private" => desc.is_private = column["value"].as_i64().unwrap() as i32,
            "init" => desc.init = column["value"].as_i64().unwrap() as i32,
            _ => {
                info!("repository unkown change target column {}", target_column)
            }
        }

        let body = RepositoryBodyContent {};
        let r = RepositoryBuilder::new(desc, body)
            .owner(repository.desc().owner().unwrap())
            .dec_id(repository.desc().dec_id().unwrap())
            // create time 需要显式指定,否则会设置now
            .create_time(repository.desc().create_time())
            .build();
        Ok(r)
    }

    async fn update(
        repository: Repository,
        stack: &Arc<SharedCyfsStack>,
        column: Value,
    ) -> BuckyResult<Repository> {
        let repository_update = Repository::update_column(repository.clone(), column)?;
        let _r = put_object(&stack, &repository_update).await?;
        let id = repository_update.desc().object_id();
        let env = stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        let (full_path, _) =
            RepositoryHelper::object_map_path(repository.author_name(), repository.name());
        let _r = env.set_with_path(full_path, &id, None, true).await?;
        let _root = env.commit().await?;
        Ok(repository_update)
    }

    async fn repository(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<Repository> {
        let buf = get_object(stack, id).await?;
        let repository = Repository::clone_from_slice(&buf)?;
        Ok(repository)
    }

    fn repo_dir(&self) -> PathBuf {
        RepositoryHelper::repo_dir(&self.author_name(), &self.name())
    }

    // repo_git_dir
    // 路径  /cyfs/data/app/cyfs-git-repos/<space>/<name>/.git
    fn repo_git_dir(&self) -> PathBuf {
        let mut dir = self.repo_dir();
        dir.push(".git");
        dir
    }

    fn branches(&self) -> BuckyResult<Vec<String>> {
        let repo_dir = self.repo_dir();
        // repository 分支信息
        let refs = git_show_ref(repo_dir)?;
        let mut branches: Vec<String> = Vec::new();
        for git_ref in refs {
            branches.push(git_ref.branch);
        }
        Ok(branches)
    }

    fn json(&self) -> Value {
        json!({
            "id": self.id(),
            "owner": self.desc().owner().unwrap(),
            "name": self.name(),
            "author_name": self.author_name(),
            "init": self.init(),
            "is_private": self.is_private(),
            "author_type": self.author_type(),
            "description": self.description(),
            "date": self.date(),
        })
    }
}

pub struct RepositoryHelper {
    pub stack: Arc<SharedCyfsStack>,
    pub author_name: String,
    pub name: String,
}

impl RepositoryHelper {
    pub fn new(stack: Arc<SharedCyfsStack>, author_name: String, name: String) -> Self {
        Self {
            stack: stack,
            author_name,
            name,
        }
    }

    pub async fn repository(&self) -> BuckyResult<Repository> {
        let result =
            RepositoryHelper::get_repository_object(&self.stack, &self.author_name, &self.name)
                .await;
        result
    }

    // 仓库的star数量
    pub async fn start_count(&self) -> BuckyResult<i32> {
        let mut count = 0;

        let key = RepositoryHelper::star_base_path(&self.author_name, &self.name);
        let env = self
            .stack
            .root_state_stub(None, Some(dec_id()))
            .create_single_op_env()
            .await?;

        let result = env.load_by_path(key).await;
        if result.is_ok() {
            loop {
                let ret = env.next(10).await?;
                if ret.len() == 0 {
                    break;
                }
                for _ in ret {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

// RootState Map
pub fn repository_object_map_path(author_name: &str, name: &str) -> String {
    format!("{}{}/{}/repo", REPOSITORY_PATH, author_name, name)
}

/// RepositoryHelper
/// static method
impl RepositoryHelper {
    pub fn repo_dir(author_name: &str, name: &str) -> PathBuf {
        let mut base_dir = cyfs_util::get_cyfs_root_path();
        base_dir.push("data");
        base_dir.push("app");
        base_dir.push(dec_id().to_string());
        base_dir.push(APP_BASE_DIR);
        if let Err(e) = std::fs::create_dir_all(&base_dir) {
            error!(
                "RepositoryHelper repo_dir create app data dir failed! dir={}, err={}",
                base_dir.display(),
                e
            );
        }
        // let mut dir = cyfs_util::get_app_data_dir(APP_BASE_DIR);
        base_dir.push(author_name);
        base_dir.push(name);
        if let Err(e) = std::fs::create_dir_all(&base_dir) {
            error!(
                "RepositoryHelper repo_dir create repo dir failed! dir={}, err={}",
                base_dir.display(),
                e
            );
        }
        base_dir
    }

    pub fn object_map_path_base(author_name: &str) -> String {
        format!("{}{}", REPOSITORY_PATH, author_name)
    }
    pub fn object_map_path(author_name: &str, name: &str) -> (String, String) {
        (
            format!("{}{}/{}/repo", REPOSITORY_PATH, author_name, name),
            format!("{}{}/{}", REPOSITORY_PATH, author_name, name),
        )
    }
    /// commit_object_map_path
    /// path /app/<space>/<name>/commit/<oid>
    pub fn commit_object_map_path(author_name: &str, name: &str) -> String {
        format!("{}{}/{}/commit", REPOSITORY_PATH, author_name, name)
    }

    pub fn star_user_key(author_name: &str, name: &str, user_name: &str) -> String {
        format!(
            "{}{}/{}/star/{}",
            REPOSITORY_PATH, author_name, name, user_name
        )
    }
    pub fn star_base_path(author_name: &str, name: &str) -> String {
        format!("{}{}/{}/star", REPOSITORY_PATH, author_name, name)
    }
    pub fn member_key(author_name: &str, name: &str, user_id: &str) -> String {
        format!(
            "{}{}/{}/member/{}",
            REPOSITORY_PATH, author_name, name, user_id
        )
    }
    pub fn member_base_path(author_name: &str, name: &str) -> String {
        format!("{}{}/{}/member", REPOSITORY_PATH, author_name, name)
    }
    pub fn merge_key(author_name: &str, name: &str, id: &str) -> String {
        format!("{}{}/{}/merge/{}", REPOSITORY_PATH, author_name, name, id)
    }
    pub fn merge_base(author_name: &str, name: &str) -> String {
        format!("{}{}/{}/merge", REPOSITORY_PATH, author_name, name)
    }

    pub async fn by_path(
        stack: &Arc<SharedCyfsStack>,
        object_map_path: &str,
    ) -> BuckyResult<Repository> {
        let env = stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        let repository_object_id = env.get_by_path(object_map_path).await?;
        info!("env.get_by_path {:?}", repository_object_id);
        if repository_object_id.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::NotFound,
                format!("Repository not found by path: {}", object_map_path),
            ));
        }
        let buf = get_object(stack, repository_object_id.unwrap()).await?;
        let repository = Repository::clone_from_slice(&buf)? as Repository;
        Ok(repository)
    }
    // pub async fn issue_by_path(stack: &Arc<SharedCyfsStack>, space: &str, name: &str, issue_id: &str, comment_id: &str) -> BuckyResult<Issue>{
    //     let env = stack.root_state_stub().create_path_op_env().await?;
    //     let issue_id = issue_id.parse::<i32>().unwrap();
    //     let comment_id =comment_id.parse::<i32>().unwrap();
    //     let key = RepositoryHelper::issue_comment_key(space,name, issue_id, comment_id);
    //     let object_id = env.get_by_path(&key).await?;

    //     let buf = get_object(stack, object_id.unwrap()).await?;
    //     let issue = Issue::clone_from_slice(&buf)? as Issue;
    //     Ok(issue)
    // }

    pub async fn member_by_path(
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        name: &str,
        repo_member_id: &str,
    ) -> BuckyResult<RepositoryMember> {
        let env = stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        let key = RepositoryHelper::member_key(space, name, repo_member_id);
        let object_id = env.get_by_path(&key).await?;

        let buf = get_object(stack, object_id.unwrap()).await?;
        let member = RepositoryMember::clone_from_slice(&buf)?;
        Ok(member)
    }

    pub async fn by_id(
        stack: &Arc<SharedCyfsStack>,
        object_id: ObjectId,
    ) -> BuckyResult<Repository> {
        let buf = get_object(stack, object_id).await?;
        let repository = Repository::clone_from_slice(&buf)? as Repository;
        Ok(repository)
    }

    /// get_repository_object
    /// quik get by path
    pub async fn get_repository_object(
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        name: &str,
    ) -> BuckyResult<Repository> {
        let (object_map_path, _) = RepositoryHelper::object_map_path(space, name);
        trace!("object_map_path {:?}", object_map_path);

        let repository = RepositoryHelper::by_path(stack, &object_map_path).await?;
        Ok(repository)
    }

    // pub async fn remove(stack: &Arc<SharedCyfsStack>, space: &str, name: &str) ->BuckyResult<Repository> {
    //     let (object_map_path,_) = RepositoryHelper::object_map_path(space, name);

    //     let repository = RepositoryHelper::by_path(stack, &object_map_path).await?;
    //     Ok(repository)
    // }
}
