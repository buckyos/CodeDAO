use crate::*;
use cyfs_base::*;

#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::BlobDescContent)]
pub struct BlobDescContent {
    blob_id: String,
    file_id: String,
}

impl DescContent for BlobDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Blob as u16
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
pub struct BlobBodyContent {}

impl BodyContent for BlobBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type BlobType = NamedObjType<BlobDescContent, BlobBodyContent>;
type BlobBuilder = NamedObjectBuilder<BlobDescContent, BlobBodyContent>;
// type BlobDesc = NamedObjectDesc<BlobDescContent>;

pub type BlobId = NamedObjectId<BlobType>;
pub type Blob = NamedObjectBase<BlobType>;

pub trait BlobObject {
    fn create(owner: ObjectId, blob_id: String, file_id: String) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn blob_id(&self) -> &String;
    fn file_id(&self) -> &String;
}

impl BlobObject for Blob {
    fn create(owner: ObjectId, blob_id: String, file_id: String) -> Self {
        let desc = BlobDescContent { blob_id, file_id };
        let body = BlobBodyContent {};

        BlobBuilder::new(desc, body)
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
    fn blob_id(&self) -> &String {
        &self.desc().content().blob_id
    }
    fn file_id(&self) -> &String {
        &self.desc().content().file_id
    }
}
