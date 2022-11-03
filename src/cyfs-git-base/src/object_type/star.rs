// use cyfs_lib::*;
use cyfs_base::*;
use crate::*;



#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::RepositoryStarDescContent)]
pub struct RepositoryStarDescContent {
    user_id: String,
    user_name: String,
    repository_id: String,
    repository_author_name: String,
    repository_name: String,
}

impl DescContent for RepositoryStarDescContent {
    fn obj_type() -> u16 {
        CustomObjType::RepositoryStar as u16
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
pub struct RepositoryStarBodyContent {
}

impl BodyContent for RepositoryStarBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type RepositoryStarType = NamedObjType<RepositoryStarDescContent, RepositoryStarBodyContent>;
type RepositoryStarBuilder = NamedObjectBuilder<RepositoryStarDescContent, RepositoryStarBodyContent>;
// type RepositoryStarDesc = NamedObjectDesc<RepositoryStarDescContent>;

pub type RepositoryStarId = NamedObjectId<RepositoryStarType>;
pub type RepositoryStar = NamedObjectBase<RepositoryStarType>;

pub trait RepositoryStarObject {
    fn create(
        owner: ObjectId, 
        user_id: String, 
        repository_id: String, 
        user_name: String,
        repository_author_name: String,
        repository_name: String,
    ) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn user_id(&self) -> &String;
    fn repository_id(&self) -> &String;
    fn user_name(&self) -> &String;
    fn repository_author_name(&self) -> &String;
    fn repository_name(&self) -> &String;
}

impl RepositoryStarObject for RepositoryStar {
    fn create(
        owner: ObjectId, 
        user_id: String, 
        repository_id: String, 
        user_name: String,
        repository_author_name: String,
        repository_name: String,
    ) -> Self {
        let desc = RepositoryStarDescContent {
            user_id,
            repository_id,
            user_name,
            repository_author_name,
            repository_name,
        };
        let body = RepositoryStarBodyContent {};

        RepositoryStarBuilder::new(desc, body)
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
