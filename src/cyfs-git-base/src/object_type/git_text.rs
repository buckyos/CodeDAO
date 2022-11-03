use cyfs_base::*;
use crate::*;



#[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
pub struct GitTextDescContent {
}

impl DescContent for GitTextDescContent {
    fn obj_type() -> u16 {
        CustomObjType::GitText as u16
    }
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
    type OwnerType = Option<ObjectId>;
    type AreaType = SubDescNone;
    type AuthorType = SubDescNone;
    type PublicKeyType = SubDescNone;
}


#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::GitTextBodyContent)]
pub struct GitTextBodyContent {
    id: String,
    header: String,
    value: String,
}

impl BodyContent for GitTextBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type GitTextType = NamedObjType<GitTextDescContent, GitTextBodyContent>;
type GitTextBuilder = NamedObjectBuilder<GitTextDescContent, GitTextBodyContent>;
// type GitTextDesc = NamedObjectDesc<GitTextDescContent>;

pub type GitTextId = NamedObjectId<GitTextType>;
pub type GitText = NamedObjectBase<GitTextType>;

pub trait GitTextObject {
    fn create(owner: Option<ObjectId>, id: String, header: String, value: String) -> Self;
    fn id(&self) -> &String;
    fn header(&self) -> &String;
    fn value(&self) -> &String;
}

impl GitTextObject for GitText {
    fn create(owner: Option<ObjectId>, id: String, header: String, value: String) -> Self {
        let desc = GitTextDescContent {
        };
        let body = GitTextBodyContent {
            id,
            header,
            value,
        };
        let mut builder = GitTextBuilder::new(desc, body);
        if owner.is_some() {
            builder = builder.owner(owner.unwrap())
        }

        builder
        .dec_id(dec_id())
        .option_create_time(None)
        .build()
    }

    fn id(&self) -> &String {
        &self.body_expect("").content().id
    }

    fn header(&self) -> &String {
        &self.body_expect("").content().header
    }

    fn value(&self) -> &String {
        &self.body_expect("").content().value
    }
}
