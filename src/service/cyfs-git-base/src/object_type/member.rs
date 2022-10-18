use cyfs_base::*;
// use cyfs_lib::*;
use std::str::FromStr;
use log::*;
use crate::*;



#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::RepositoryMemberDescContent)]
pub struct RepositoryMemberDescContent {
    user_id: String,
    repository_id: String,
    role: String,
    user_name: String,
    repository_author_name: String,
    repository_name: String,
}

impl DescContent for RepositoryMemberDescContent {
    fn obj_type() -> u16 {
        CustomObjType::RepositoryMember as u16
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
pub struct RepositoryMemberBodyContent {
}

impl BodyContent for RepositoryMemberBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type RepositoryMemberType = NamedObjType<RepositoryMemberDescContent, RepositoryMemberBodyContent>;
type RepositoryMemberBuilder = NamedObjectBuilder<RepositoryMemberDescContent, RepositoryMemberBodyContent>;
// type RepositoryMemberDesc = NamedObjectDesc<RepositoryMemberDescContent>;

pub type RepositoryMemberId = NamedObjectId<RepositoryMemberType>;
pub type RepositoryMember = NamedObjectBase<RepositoryMemberType>;

pub trait RepositoryMemberObject {
    fn create(
        owner: ObjectId, 
        user_id: String, 
        repository_id: String, 
        role: RepositoryMemberRole, 
        user_name: String,
        repository_author_name: String,
        repository_name: String,
    ) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn user_id(&self) -> &String;
    fn repository_id(&self) -> &String;
    // TODO member role emun
    fn role(&self) -> &String;
    fn user_name(&self) -> &String;
    fn repository_author_name(&self) -> &String;
    fn repository_name(&self) -> &String;
}

impl RepositoryMemberObject for RepositoryMember {
    fn create(
        owner: ObjectId, 
        user_id: String, 
        repository_id: String, 
        role: RepositoryMemberRole, 
        user_name: String,
        repository_author_name: String,
        repository_name: String,
    ) -> Self {
        let desc = RepositoryMemberDescContent {
            user_id,
            repository_id,
            role: role.to_string(),
            user_name,
            repository_author_name,
            repository_name,
        };
        let body = RepositoryMemberBodyContent {};

        RepositoryMemberBuilder::new(desc, body)
            .owner(owner)
            .dec_id(dec_id())
            .option_create_time(None)
            .build()
    }
    fn id(&self) -> String {
        self.desc().calculate_id().to_string()
    }
    fn owner(&self) -> String {
        self.desc().owner().unwrap().to_string()
    }
    fn date(&self) -> u64 {
        bucky_time_to_js_time(self.desc().create_time())
    }
    fn user_id(&self) -> &String {
        &self.desc().content().user_id
    }
    fn repository_id(&self) -> &String {
        &self.desc().content().repository_id
    }
    fn role(&self) -> &String {
        &self.desc().content().role
    }
    fn user_name(&self) -> &String {
        &self.desc().content().user_name
    }
    fn repository_author_name(&self) -> &String {
        &self.desc().content().repository_author_name
    }
    fn repository_name(&self) -> &String {
        &self.desc().content().repository_name
    }
}

pub enum RepositoryMemberRole {
    Read,
    Write,
    Admin,
}

impl FromStr for RepositoryMemberRole {
    type Err = BuckyError;
    fn from_str(input: &str) -> BuckyResult<RepositoryMemberRole> {
        match input {
            "read"  => Ok(RepositoryMemberRole::Read),
            "write"  => Ok(RepositoryMemberRole::Write),
            "admin"  => Ok(RepositoryMemberRole::Admin),
            _      => {
                error!("error RepositoryMemberRole value");
                Err(BuckyError::new(BuckyErrorCode::NotFound, "error RepositoryMemberRole value"))
            },
        }
    }
}
impl RepositoryMemberRole {
    pub fn to_string(&self) ->  String{
        let result = match self {
            RepositoryMemberRole::Read  => "read",
            RepositoryMemberRole::Write  => "write",
            RepositoryMemberRole::Admin  => "admin",
        };
        result.to_string()
    }

    pub fn push_allow(&self) -> bool {
        if let RepositoryMemberRole::Write  = self  {
            true
        } else if let RepositoryMemberRole::Admin  = self  {
            true
        } else {
            false
        }
    }
}