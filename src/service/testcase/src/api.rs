use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_core::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use serde_json::*;
use std::str::FromStr;

pub struct Api {
    stack: Arc<SharedCyfsStack>,
    route: String,
    req_data: Option<String>,
}

// trait ApiMethod {
//     pub async fn user_init(&self);
// }

impl Api {
    pub fn new(
        stack: Arc<SharedCyfsStack>,
        route: impl Into<String>,
        req_data: Option<String>,
    ) -> Self {
        Self {
            stack,
            route: route.into(),
            req_data,
        }
    }
}

impl Api {
    pub async fn user_init(&self) {
        let data = if let Some(ref data) = self.req_data {
            data
        } else {
            r#"
            {
                "name": "sunxinle",
                "email":"sunxinle@buckyos.com"
            }"#
        };
        post_object(&self.stack, &self.route, data).await;
    }

    pub async fn common(&self) {
        let data = self.req_data.clone().expect("empty data");
        post_object(&self.stack, &self.route, &data).await;
    }
}

pub async fn test_dec_id(stack: &Arc<SharedCyfsStack>) {
    let owner = ObjectId::from_str("5r4MYfFPKMeHa1fec7dHKmBfowySBfVFvRQvKB956dnF").unwrap();
    let app_dec_id = DecApp::generate_id(owner.clone(), CODE_DAO_SERVICE_NAME);
    println!("dec id {:?}", app_dec_id);
}

pub async fn debug_info(stack: &Arc<SharedCyfsStack>, _route: &str) {
    let info = stack
        .util_service()
        .get_device_static_info(UtilGetDeviceStaticInfoOutputRequest {
            common: UtilOutputRequestCommon {
                ..Default::default()
            },
        })
        .await
        .unwrap();

    println!("info  {:#?}", info);
}

pub async fn repository_merge_compare(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {"target":"master","origin":"test","author_name":"sunxinle","name":"0309"}"#;
    post_object(stack, route, data).await;
}

pub async fn repository_issue_list(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "name":"aaa",
        "author_name":"sunxinle"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_issue_new(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "name":"aaa",
        "author_name":"sunxinle",
        "title":"issue title",
        "content":"issue content"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_commit(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "branch": "master",
        "name":"aaa",
        "author_name":"sunxinle",
        "commit_id": "0ac9b995f19bb3e2f3d9fbfbf6428db11313f119"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_commits(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "branch": "master",
        "name":"aaa",
        "author_name":"sunxinle"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_home(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "id": "",
        "path": "/",
        "branch": "master",
        "hash": "",
        "name":"aaa",
        "author_name":"sunxinle"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_file(stack: &Arc<SharedCyfsStack>, route: &str, file: Option<String>) {
    let data = if file.is_some() {
        r#"
        {
            "id": "",
            "path": "test.txt",
            "branch": "master",
            "hash": "",
            "name":"aaa",
            "author_name":"sunxinle"
        }"#
    } else {
        r#"
        {"name":"20220902newtest","owner":"sunxinle001","author_name":"sunxinle001","file_name":"","path":"","hash":"","branch":"master"}"#
    };
    post_object(stack, route, data).await;
}

pub async fn repository_push_head(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "name":"aaa",
        "author_name":"sunxinle"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_delete(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "name":"aaa",
        "author_name":"sunxinle"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_log_graph(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {
        "name":"0420",
        "author_name":"sunxinle",
        "branch": "master"
    }"#;
    post_object(stack, route, data).await;
}

pub async fn repository_list(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"
    {}"#;
    post_object(stack, route, data).await;
}

pub async fn repository_new(stack: &Arc<SharedCyfsStack>, route: &str) {
    let data = r#"{"name":"20220922_1714","description":"aaa","is_private":0,"author_type":"user","author_name":"sunxinle"}"#;
    post_object(stack, route, data).await;
}

pub async fn repository_private_new(stack: &Arc<SharedCyfsStack>) {
    let data = r#"{"name":"privatetest","description":"aaa","is_private":1,"author_type":"user","author_name":"sunxinle"}"#;
    let route = "repo/new";
    post_object(stack, route, data).await;
}

pub async fn organization_new(stack: &Arc<SharedCyfsStack>) {
    let data = r#"
    {
        "name": "test_group",
        "email":"sunxinle@buckyos.com"
    }"#;
    let route = "organization/new";
    post_object(stack, route, data).await;
}

pub async fn organization_list(stack: &Arc<SharedCyfsStack>) {
    let data = r#"
    {}"#;
    let route = "organization/list";
    post_object(stack, route, data).await;
}

pub async fn user_get_by_name(stack: &Arc<SharedCyfsStack>) {
    let data = r#"
    {
        "name":"sunxinle"
    }"#;
    let route = "user/getByName";
    post_object(stack, route, data).await;
}

pub async fn user_list(stack: &Arc<SharedCyfsStack>) {
    let data = r#"
    {
    }"#;
    let route = "user/list";
    post_object(stack, route, data).await;
}

pub async fn user_check_init(stack: &Arc<SharedCyfsStack>) {
    let data = r#"
    {
    }"#;
    let route = "user/checkInit";
    post_object(stack, route, data).await;
}

pub async fn user_setting(stack: &Arc<SharedCyfsStack>, email: Option<String>) {
    let data = if email.is_none() {
        r#"
        {
            "name": "sunxinle",
            "email":"sunxinle@buckyos.com"
        }"#
        .to_string()
    } else {
        format!(
            r#"{{"name": "sunxinle","email": "{}@bukcyos.com"}}"#,
            email.unwrap()
        )
    };
    let route = "user/setting";
    post_object(stack, route, &data).await;
}

pub async fn post_object(stack: &Arc<SharedCyfsStack>, route: &str, data: &str) {
    let owner = get_owner(&stack).await;

    println!("route: {:#?}", route);
    println!("post object request data: {:#?}", data);
    let text = GitText::create(
        Some(owner),
        route.to_string(),
        "header".to_string(),
        data.to_string(),
    );

    let req_path = Some(DEC_APP_HANDLER.to_string());

    let r = stack
        .non_service()
        .post_object(NONPostObjectOutputRequest {
            common: NONOutputRequestCommon {
                req_path,
                source: None,
                dec_id: Some(dec_id()),
                level: NONAPILevel::Router,
                // target: Some(get_local_device(stack)),
                target: None,
                flags: 0,
            },
            object: NONObjectInfo {
                object_id: text.desc().object_id().clone(),
                object_raw: text.to_vec().unwrap(),
                object: None,
            },
        })
        .await
        .unwrap();
    println!("post object {:?}", r.to_string());

    let buf = r.object.unwrap().object_raw;
    let text = GitText::clone_from_slice(&buf).unwrap() as GitText;

    let val: Value = serde_json::from_str(text.value()).unwrap();
    let result = serde_json::to_string_pretty(&val).unwrap();
    println!("{}", result);
}
