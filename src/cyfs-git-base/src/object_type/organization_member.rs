// use cyfs_lib::*;
use cyfs_base::*;
use crate::*;



#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::OrganizationMemberDescContent)]
pub struct OrganizationMemberDescContent {
    organization_name: String,
    user_id: String,
    user_name: String,
    role: String,
}

impl DescContent for OrganizationMemberDescContent {
    fn obj_type() -> u16 {
        CustomObjType::OrganizationMember as u16
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
pub struct OrganizationMemberBodyContent {
}

impl BodyContent for OrganizationMemberBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type OrganizationMemberType = NamedObjType<OrganizationMemberDescContent, OrganizationMemberBodyContent>;
type OrganizationMemberBuilder = NamedObjectBuilder<OrganizationMemberDescContent, OrganizationMemberBodyContent>;
// type OrganizationMemberDesc = NamedObjectDesc<OrganizationMemberDescContent>;

pub type OrganizationMemberId = NamedObjectId<OrganizationMemberType>;
pub type OrganizationMember = NamedObjectBase<OrganizationMemberType>;

pub trait OrganizationMemberObject {
    fn create(
        owner: ObjectId, 
        organization_name: String,
        user_id: String, 
        user_name: String,
        role: String, 
    ) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn user_id(&self) -> &String;
    fn organization_name(&self) -> &String;
    fn role(&self) -> &String;
    fn user_name(&self) -> &String;
}

impl OrganizationMemberObject for OrganizationMember {
    fn create(
        owner: ObjectId, 
        organization_name: String,
        user_id: String, 
        user_name: String,
        role: String, 
    ) -> Self {
        let desc = OrganizationMemberDescContent {
            organization_name,
            user_id,
            user_name,
            role,
        };
        let body = OrganizationMemberBodyContent {};

        OrganizationMemberBuilder::new(desc, body)
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
    fn organization_name(&self) -> &String {
        &self.desc().content().organization_name
    }
    fn role(&self) -> &String {
        &self.desc().content().role
    }
    fn user_name(&self) -> &String {
        &self.desc().content().user_name
    }
}
