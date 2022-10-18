use cyfs_base::*;
use cyfs_lib::*;
use async_std::sync::Arc;
use async_trait::async_trait;
use log::*;
use crate::*;


#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::RepositoryBranchDescContent)]
pub struct RepositoryBranchDescContent {
    repository_author_name: String,
    repository_name: String,
    ref_name: String,
    ref_hash: String,
}

impl DescContent for RepositoryBranchDescContent {
    fn obj_type() -> u16 {
        CustomObjType::RepositoryBranch as u16
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
pub struct RepositoryBranchBodyContent {
}

impl BodyContent for RepositoryBranchBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type RepositoryBranchType = NamedObjType<RepositoryBranchDescContent, RepositoryBranchBodyContent>;
type RepositoryBranchBuilder = NamedObjectBuilder<RepositoryBranchDescContent, RepositoryBranchBodyContent>;
// type RepositoryBranchDesc = NamedObjectDesc<RepositoryBranchDescContent>;

pub type RepositoryBranchId = NamedObjectId<RepositoryBranchType>;
pub type RepositoryBranch = NamedObjectBase<RepositoryBranchType>;

pub trait RepositoryBranchObject {
    fn create(owner: ObjectId, repository_author_name: String, repository_name: String, ref_name: String, ref_hash: String) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn repository_author_name(&self) -> &String;
    fn repository_name(&self) -> &String;
    fn ref_name(&self) -> &String;
    fn ref_hash(&self) -> &String;
}

impl RepositoryBranchObject for RepositoryBranch {
    fn create(owner: ObjectId, repository_author_name: String, repository_name: String, ref_name: String, ref_hash: String) -> Self {

        let desc = RepositoryBranchDescContent {
            repository_author_name,
            repository_name,
            ref_name,
            ref_hash,
        };
        let body = RepositoryBranchBodyContent {};

        RepositoryBranchBuilder::new(desc, body)
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
    fn repository_author_name(&self) -> &String {
        &self.desc().content().repository_author_name
    }
    fn repository_name(&self) -> &String {
        &self.desc().content().repository_name
    }
    fn ref_name(&self) -> &String {
        &self.desc().content().ref_name
    }
    fn ref_hash(&self) -> &String {
        &self.desc().content().ref_hash
    }
}


pub fn refs_map_path(author_name: &str, repo_name: &str, ref_name: &str) -> String {
    format!("{}{}/{}/refs/{}", REPOSITORY_PATH, author_name, repo_name, ref_name)
}

pub fn refs_map_path_base(author_name: &str, repo_name: &str) -> String {
    format!("{}{}/{}/refs", REPOSITORY_PATH, author_name, repo_name)
}


#[async_trait]
pub trait RepositoryBranchUtil {
    async fn insert_ref(&self, stack: &Arc<SharedCyfsStack>) -> BuckyResult<()>;
    async fn read_refs(stack: &Arc<SharedCyfsStack>, space: &str, name: &str) -> BuckyResult<Vec<GitRef>>;
    async fn read_ref(stack: &Arc<SharedCyfsStack>, space: &str, name: &str, ref_name: &str) -> BuckyResult<RepositoryBranch>;
}

#[async_trait]
impl RepositoryBranchUtil for RepositoryBranch {
    async fn insert_ref(&self, stack: &Arc<SharedCyfsStack>) -> BuckyResult<()> {
        let id = self.desc().calculate_id();
        let _r = put_object(stack, self).await?;


        let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
        let full_path = refs_map_path(
            self.repository_author_name(),
            self.repository_name(),
            self.ref_name(),
        );

        let _r = env.set_with_path(full_path.clone(), &id, None, true).await?;
        let _ = env.commit().await?;

        info!("insert repository ref ok {}: {}", full_path, self.ref_hash());
        Ok(())
    }

    async fn read_refs(stack: &Arc<SharedCyfsStack>, space: &str, name: &str) -> BuckyResult<Vec<GitRef>> {
        let mut branches: Vec<RepositoryBranch> = vec![];
        let mut resp_refs: Vec<GitRef> = vec![];

        let refs_base_path = refs_map_path_base(
            space,
            name,
        );
        let env = stack.root_state_stub(None, Some(dec_id())).create_single_op_env().await?;
        let result = env.load_by_path(refs_base_path).await;
        if result.is_err() {
            return Ok(resp_refs)
        }

        let ret = env.list().await?;
        for item in ret {
            let (_, id) = item.into_map_item();
            let buf = get_object(stack, id).await?;
            let repository_branch = RepositoryBranch::clone_from_slice(&buf)?;
            branches.push(repository_branch);
        }

        for branch in branches {
            resp_refs.push(GitRef{
                ref_name: format!("refs/heads/{}", branch.ref_name()),
                branch: branch.ref_name().to_string(),
                hash: branch.ref_hash().to_string(),
            })
        }

        Ok(resp_refs)
    }


    // get single branch info by branch name
    async fn read_ref(stack: &Arc<SharedCyfsStack>, space: &str, name: &str, ref_name: &str) -> BuckyResult<RepositoryBranch> {

        let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
        let full_path = refs_map_path(space, name, ref_name);
        let id = env.get_by_path(full_path).await?.unwrap();
        // if id.is_none

        let buf = get_object(stack, id).await?;
        let repository_branch = RepositoryBranch::clone_from_slice(&buf)?;
        Ok(repository_branch)
    }
}
