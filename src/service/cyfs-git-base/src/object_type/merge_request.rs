// use cyfs_lib::*;
use cyfs_base::*;
use crate::*;



#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::MergeRequestDescContent)]
pub struct MergeRequestDescContent {
    title: String,
    origin_branch: String,
    target_branch: String,
    merge_type: String,
    status: String,
    repository_author_name: String,
    repository_name: String,
    user_name: String,
}

impl DescContent for MergeRequestDescContent {
    fn obj_type() -> u16 {
        CustomObjType::MergeRequest as u16
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
pub struct MergeRequestBodyContent {
}

impl BodyContent for MergeRequestBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type MergeRequestType = NamedObjType<MergeRequestDescContent, MergeRequestBodyContent>;
type MergeRequestBuilder = NamedObjectBuilder<MergeRequestDescContent, MergeRequestBodyContent>;
// type MergeRequestDesc = NamedObjectDesc<MergeRequestDescContent>;

pub type MergeRequestId = NamedObjectId<MergeRequestType>;
pub type MergeRequest = NamedObjectBase<MergeRequestType>;

pub trait MergeRequestObject {
    fn create(
        owner: ObjectId,
        title: String,
        origin_branch: String,
        target_branch: String,
        merge_type: String,
        status: String,
        repository_author_name: String,
        repository_name: String,
        user_name: String,
    ) -> Self ;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn title(&self) -> &String;
    fn origin_branch(&self) -> &String;
    fn target_branch(&self) -> &String;
    fn merge_type(&self) -> &String;
    fn status(&self) -> &String;
    fn repository_author_name(&self) -> &String;
    fn repository_name(&self) -> &String;
    fn user_name(&self) -> &String;
}

impl MergeRequestObject for MergeRequest {
    fn create(
        owner: ObjectId,
        title: String,
        origin_branch: String,
        target_branch: String,
        merge_type: String,
        status: String,
        repository_author_name: String,
        repository_name: String,
        user_name: String,
    ) -> Self {
        let desc = MergeRequestDescContent {
            title,
            origin_branch,
            target_branch,
            merge_type,
            status,
            repository_author_name,
            repository_name,
            user_name,
        };
        let body = MergeRequestBodyContent {};

        MergeRequestBuilder::new(desc, body)
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
    fn title(&self) -> &String {
        &self.desc().content().title
    }
    fn origin_branch(&self) -> &String {
        &self.desc().content().origin_branch
    }
    fn target_branch(&self) -> &String {
        &self.desc().content().target_branch
    }
    fn merge_type(&self) -> &String {
        &self.desc().content().merge_type
    }
    fn status(&self) -> &String {
        &self.desc().content().status
    }
    fn repository_author_name(&self) -> &String {
        &self.desc().content().repository_author_name
    }
    fn repository_name(&self) -> &String {
        &self.desc().content().repository_name
    }
    fn user_name(&self) -> &String {
        &self.desc().content().user_name
    }
}
