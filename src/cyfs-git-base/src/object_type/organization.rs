use cyfs_base::*;
use crate::*;



// #[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::OrganizationDescContent)]
pub struct OrganizationDescContent {
    name: String,
    description: String,
    email: String,
    avatar: String,
}

impl DescContent for OrganizationDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Organization as u16
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
pub struct OrganizationBodyContent {
}

impl BodyContent for OrganizationBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type OrganizationType = NamedObjType<OrganizationDescContent, OrganizationBodyContent>;
type OrganizationBuilder = NamedObjectBuilder<OrganizationDescContent, OrganizationBodyContent>;
// type OrganizationDesc = NamedObjectDesc<OrganizationDescContent>;

pub type OrganizationId = NamedObjectId<OrganizationType>;
pub type Organization = NamedObjectBase<OrganizationType>;

pub trait OrganizationObject {
    fn create(owner: ObjectId, name: String, description: String, email: String, avatar: String) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn name(&self) -> &String;
    fn description(&self) -> &String;
    fn email(&self) -> &String;
    fn avatar(&self) -> &String;
}

impl OrganizationObject for Organization {
    fn create(owner: ObjectId, name: String, description: String, email: String, avatar: String) -> Self {

        let desc = OrganizationDescContent {
            name,
            description,
            email,
            avatar,
        };
        let body = OrganizationBodyContent {};

        OrganizationBuilder::new(desc, body)
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
    fn owner(&self) -> String {
        self.desc().owner().unwrap().to_string()
    }
    fn name(&self) -> &String {
        &self.desc().content().name
    }
    fn description(&self) -> &String {
        &self.desc().content().description
    }
    fn email(&self) -> &String {
        &self.desc().content().email
    }
    fn avatar(&self) -> &String {
        &self.desc().content().avatar
    }
}
