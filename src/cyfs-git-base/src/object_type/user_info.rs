use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_lib::*;
use serde_json::{json, Value};

// #[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::UserInfoDescContent)]
pub struct UserInfoDescContent {
    pub name: String,
    pub email: String,
}

impl DescContent for UserInfoDescContent {
    fn obj_type() -> u16 {
        CustomObjType::UserInfo as u16
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
pub struct UserInfoBodyContent {}

impl BodyContent for UserInfoBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type UserInfoType = NamedObjType<UserInfoDescContent, UserInfoBodyContent>;
type UserInfoBuilder = NamedObjectBuilder<UserInfoDescContent, UserInfoBodyContent>;
// type UserInfoDesc = NamedObjectDesc<UserInfoDescContent>;

pub type UserInfoId = NamedObjectId<UserInfoType>;
pub type UserInfo = NamedObjectBase<UserInfoType>;

pub trait UserInfoObject {
    fn create(owner: ObjectId, name: String, email: String) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn name(&self) -> &String;
    fn email(&self) -> &String;
}

impl UserInfoObject for UserInfo {
    fn create(owner: ObjectId, name: String, email: String) -> Self {
        let desc = UserInfoDescContent {
            name,
            email,
        };
        let body = UserInfoBodyContent {};

        UserInfoBuilder::new(desc, body)
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
    fn email(&self) -> &String {
        &self.desc().content().email
    }
}

pub trait UserInfoObjectUtil {
    fn json(&self) -> Value;
}
impl UserInfoObjectUtil for UserInfo {
    fn json(&self) -> Value {
        json!({
            "id": self.id(),
            "owner_id": self.owner(),
            "name": self.name(),
            "email": self.email(),
            "date": self.date(),
            "type": "user",
        })
    }
}

pub struct UserHelper {}
impl UserHelper {
    pub async fn user(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<UserInfo> {
        let buf = get_object(&stack, id).await?;
        let user = UserInfo::clone_from_slice(&buf)?;
        Ok(user)
    }

    pub async fn get_current_user(stack: &Arc<SharedCyfsStack>) -> BuckyResult<UserInfo> {
        let owner_id = get_owner(stack).await.to_string();
        let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
        let result = env.get_by_key(USER_LIST_PATH, &owner_id).await?;
        if result.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::NotFound,
                format!("user[{}] not found", owner_id),
            ));
        }
        let id = result.unwrap();
        Ok(Self::user(stack, id).await?)
    }

    pub async fn json(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<Value> {
        let user = Self::user(stack, id).await?;
        Ok(user.json())
    }
}
