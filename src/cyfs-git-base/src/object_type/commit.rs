use async_std::sync::Arc;
use async_trait::async_trait;
use cyfs_base::*;
use cyfs_lib::*;
use std::collections::HashMap;
// use log::*;
use crate::*;

// #[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmp&tyDecode)]
#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::CommitDescContent)]
pub struct CommitDescContent {
    object_id: String,
    parents: Vec<String>,
    tree_id: String,
    payload: String,
    author: Option<CommitSignature>,
    committer: Option<CommitSignature>,
    //parent2: String,
}

#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::CommitSignature)]
pub struct CommitSignature {
    pub name: String,
    pub email: String,
    pub when: String,
}

impl DescContent for CommitDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Commit as u16
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
pub struct CommitBodyContent {}

impl BodyContent for CommitBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type CommitType = NamedObjType<CommitDescContent, CommitBodyContent>;
type CommitBuilder = NamedObjectBuilder<CommitDescContent, CommitBodyContent>;
// type CommitDesc = NamedObjectDesc<CommitDescContent>;

pub type CommitId = NamedObjectId<CommitType>;
pub type Commit = NamedObjectBase<CommitType>;

pub trait CommitObject {
    fn create(
        owner: ObjectId,
        object_id: String,
        parents: Vec<String>,
        tree_id: String,
        payload: String,
        author: Option<CommitSignature>,
        committer: Option<CommitSignature>,
    ) -> Self;
    fn id(&self) -> String;
    fn object_id(&self) -> &String;
    fn parents(&self) -> &Vec<String>;
    fn tree_id(&self) -> &String;
    fn payload(&self) -> &String;
    fn author(&self) -> &Option<CommitSignature>;
    fn committer(&self) -> &Option<CommitSignature>;
}

impl CommitObject for Commit {
    fn create(
        owner: ObjectId,
        object_id: String,
        parents: Vec<String>,
        tree_id: String,
        payload: String,
        author: Option<CommitSignature>,
        committer: Option<CommitSignature>,
    ) -> Self {
        let desc = CommitDescContent {
            object_id,
            parents,
            tree_id,
            payload,
            author,
            committer,
        };
        let body = CommitBodyContent {};

        CommitBuilder::new(desc, body)
            .owner(owner)
            .dec_id(dec_id())
            .option_create_time(None)
            .build()
    }
    fn id(&self) -> String {
        self.desc().calculate_id().to_string()
    }
    fn object_id(&self) -> &String {
        &self.desc().content().object_id
    }
    fn parents(&self) -> &Vec<String> {
        &self.desc().content().parents
    }
    fn tree_id(&self) -> &String {
        &self.desc().content().tree_id
    }
    fn payload(&self) -> &String {
        &self.desc().content().payload
    }
    fn author(&self) -> &Option<CommitSignature> {
        &self.desc().content().author
    }
    fn committer(&self) -> &Option<CommitSignature> {
        &self.desc().content().committer
    }
}

// get commits by target branch
pub async fn commits_of_branch(
    _stack: &Arc<SharedCyfsStack>,
    space: &str,
    name: &str,
    branch_name: &str,
) -> BuckyResult<HashMap<String, Arc<GitCommit>>> {
    let repo_dir = RepositoryHelper::repo_dir(space, name);
    let commits = git_commits(repo_dir, branch_name)?;
    // let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
    let mut result: HashMap<String, Arc<GitCommit>> = HashMap::new();
    // info!("start to get commit objects");
    for commit in commits {
        let current_oid = commit.object_id.clone();
        //     let key = commit_object_map_key(space, name, &current_oid);
        //     let commit_object_id = env.get_by_path(&key).await?;
        //     if commit_object_id.is_none() {
        //         error!("error get commit object id, {}", current_oid);
        //         return Err(BuckyError::new(BuckyErrorCode::NotFound, "not found"))
        //     }
        //     let buf = get_object(stack, commit_object_id.unwrap()).await?;
        //     let commit = Commit::clone_from_slice(&buf)?;
        result.insert(current_oid, Arc::new(commit));
    }
    // info!("commit len {}", result.len());
    Ok(result)
}

#[async_trait]
pub trait CommitUtil {
    async fn tree(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
    ) -> BuckyResult<Tree>;
}

#[async_trait]
impl CommitUtil for Commit {
    async fn tree(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
    ) -> BuckyResult<Tree> {
        let tree = Tree::get_tree_object(stack, space, repo_name, self.tree_id()).await?;
        Ok(tree)
    }
}

#[cfg(test)]
mod main_tests {
    use log::*;
    use std::collections::HashMap;
    use std::fmt::format;
    use std::rc::Rc;
    use std::str::FromStr;

    use super::*;

    async fn test_base() -> BuckyResult<Arc<SharedCyfsStack>> {
        cyfs_debug::CyfsLoggerBuilder::new_app("cyfs-git-test")
            .level("info")
            .console("info")
            .enable_bdt(Some("error"), Some("error"))
            .module("cyfs_lib", Some("error"), Some("error"))
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
        info!("dec {}", dec_id());
        let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
        // let stack = Arc::new(SharedCyfsStack::open_runtime(Some(dec_id())).await.unwrap());
        stack.wait_online(None).await?;
        Ok(stack)
    }
    const SPACE: &'static str = "sunxinle001";
    const REPO_NAME: &'static str = "20220902newtest";

    #[async_std::test]
    async fn test_commits_of_branch() -> BuckyResult<()> {
        let stack = test_base().await?;
        let commits = commits_of_branch(&stack, SPACE, REPO_NAME, "master").await?;

        info!("commits {:?}", commits.len());

        Ok(())
    }
}
