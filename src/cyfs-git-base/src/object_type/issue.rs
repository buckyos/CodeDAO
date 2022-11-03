use cyfs_base::*;
use cyfs_lib::SharedCyfsStack;
use serde_json::{json, Value};
use std::sync::Arc;
// use cyfs_lib::*;
use crate::*;
use async_trait::async_trait;

#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::IssueDescContent)]
pub struct IssueDescContent {
    title: String,
    content: String,
    status: String,
    issue_type: String,
    user_name: String,
    id_in_repo: String,
}

impl DescContent for IssueDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Issue as u16
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
pub struct IssueBodyContent {}

impl BodyContent for IssueBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type IssueType = NamedObjType<IssueDescContent, IssueBodyContent>;
type IssueBuilder = NamedObjectBuilder<IssueDescContent, IssueBodyContent>;
// type IssueDesc = NamedObjectDesc<IssueDescContent>;

pub type IssueId = NamedObjectId<IssueType>;
pub type Issue = NamedObjectBase<IssueType>;

pub trait IssueObject {
    fn create(
        owner: ObjectId,
        title: String,
        content: String,
        status: String,
        issue_type: String,
        user_name: String,
        id_in_repo: String,
    ) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn owner(&self) -> String;
    fn title(&self) -> &String;
    fn content(&self) -> &String;
    fn status(&self) -> &String;
    fn issue_type(&self) -> &String;
    fn user_name(&self) -> &String;
    fn id_in_repo(&self) -> &String;
}

impl IssueObject for Issue {
    fn create(
        owner: ObjectId,
        title: String,
        content: String,
        status: String,
        issue_type: String,
        user_name: String,
        id_in_repo: String,
    ) -> Self {
        let desc = IssueDescContent {
            title,
            content,
            status,
            issue_type,
            user_name,
            id_in_repo,
        };
        let body = IssueBodyContent {};

        IssueBuilder::new(desc, body)
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
    fn content(&self) -> &String {
        &self.desc().content().content
    }
    fn status(&self) -> &String {
        &self.desc().content().status
    }
    fn issue_type(&self) -> &String {
        &self.desc().content().issue_type
    }
    fn user_name(&self) -> &String {
        &self.desc().content().user_name
    }
    fn id_in_repo(&self) -> &String {
        &self.desc().content().id_in_repo
    }
}

pub fn issue_topic_key(author_name: &str, name: &str, topic_id: &str) -> String {
    let topic_base = issue_map_base_path(author_name, name);
    format!("{}/{}", topic_base, topic_id)
}

// topic base path
pub fn issue_map_base_path(author_name: &str, name: &str) -> String {
    let full_repo_name = format!("{}/{}", author_name, name);
    format!("{}{}/issue", REPOSITORY_PATH, full_repo_name)
}

pub fn issue_comment_base(author_name: &str, name: &str, topic_id: &str) -> String {
    let full_repo_name = format!("{}/{}", author_name, name);
    format!(
        "{}{}/issue_comment/{}",
        REPOSITORY_PATH, full_repo_name, topic_id
    )
}

pub fn issue_comment_key(
    author_name: &str,
    name: &str,
    topic_id: &str,
    comment_id: &str,
) -> String {
    let comment_base = issue_comment_base(author_name, name, topic_id);
    format!("{}/{}", comment_base, comment_id)
}

#[async_trait]
pub trait IssueObjectUtil {
    async fn issue(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<Issue>;
    fn json(&self) -> Value;
    fn close(issue: Issue) -> BuckyResult<Issue>;
}

#[async_trait]
impl IssueObjectUtil for Issue {
    /// get issue by object_id
    async fn issue(stack: &Arc<SharedCyfsStack>, id: ObjectId) -> BuckyResult<Issue> {
        let buf = get_object(stack, id).await?;
        let issue = Issue::clone_from_slice(&buf)?;
        Ok(issue)
    }

    fn json(&self) -> Value {
        json!({
            "id": self.id(),
            "title": self.title(),
            "object_id": self.id(),
            "date": self.date(),
            "user_id": self.owner(),
            "content": self.content(),
            "status": self.status(),
            "user_name": self.user_name(),
        })
    }

    fn close(issue: Issue) -> BuckyResult<Issue> {
        let desc = IssueDescContent {
            title: issue.title().to_string(),
            content: issue.content().to_string(),
            status: "close".to_string(),
            issue_type: issue.issue_type().to_string(),
            user_name: issue.user_name().to_string(),
            id_in_repo: issue.id_in_repo().to_string(),
	};
	let body = IssueBodyContent {};
	let owner = issue.desc().owner().unwrap();
	let dec_id = issue.desc().dec_id().unwrap();
	let create_time = issue.desc().create_time();
	let new_issue = IssueBuilder::new(desc, body)
	    .owner(owner)
	    .dec_id(dec_id)
	    .create_time(create_time)
	    .build();
	Ok(new_issue)
    }
}
