
use cyfs_lib::*;
use cyfs_base::*;
use crate::*;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostObjectCommonResponseData {
    pub err: bool,
    pub status: i32,
    pub data: Option<Value>,
    pub msg: Option<String>,
}

impl PostObjectCommonResponseData {
    pub fn parse(content: &str) -> BuckyResult<Self> {
        let data: PostObjectCommonResponseData = serde_json::from_str(content).map_err(transform_err)?;
        Ok(data)
    }
}


pub fn transform_err(e: serde_json::Error) -> BuckyError {
    BuckyError::new(BuckyErrorCode::InvalidParam, format!("参数错误{:?}", e))
}


/// decode_text
/// 解析 handler event
pub fn decode_text(param: &RouterHandlerPostObjectRequest) -> BuckyResult<(String, String, ObjectId, String)> {
    let request = &param.request.object.object_raw;
    let req_text = GitText::clone_from_slice(request).unwrap();
    // println!("request param  {:?}", );
    let id = req_text.id().to_string();
    let value = req_text.value().to_string();
    let owner = req_text.desc().owner().unwrap();
    let object_id = req_text.desc().calculate_id().to_string();

    Ok((id, value, owner, object_id))
}

pub fn success(value: serde_json::Value ) -> NONPostObjectInputResponse {
    let resp_body = json!({
        "err": false,
        "status": 0, // tmp
        "data": value,
    }).to_string();
    success_proxy(resp_body)
}

pub fn success_proxy(resp_body: String ) -> NONPostObjectInputResponse {
    info!("response data: {:?}", resp_body);

    let text = GitText::create(
        None,
        "response".to_string(),
        "response header".to_string(), 
        resp_body);

    NONPostObjectInputResponse {
        object: Some(NONObjectInfo{
            object_id:  text.desc().object_id().clone(),
            object_raw: text.to_vec().unwrap(),
            object: None,
        }),
    }
}

pub fn failed(message: &str) -> NONPostObjectInputResponse {
    let resp_body = json!({
        "err": true,
        "status": 12, // tmp
        "msg": message.to_string(),
    }).to_string();
    let text = GitText::create(
        None,
        "response".to_string(),
        "response header".to_string(), 
        resp_body);

    NONPostObjectInputResponse {
        object: Some(NONObjectInfo{
            object_id:  text.desc().object_id().clone(),
            object_raw: text.to_vec().unwrap(),
            object: None,
        }),
    }
}


// acl handler response
pub fn acl_reject() -> RouterHandlerAclResult{
    RouterHandlerAclResult {
        action: RouterHandlerAction::Reject,
        request: None,
        response: Some(Ok(AclHandlerResponse {
            access: AclAccess::Accept,
        })),
    }
}


// acl handler response
pub fn acl_pass() -> RouterHandlerAclResult{
    RouterHandlerAclResult {
        action: RouterHandlerAction::Response,
        request: None,
        response: Some(Ok(AclHandlerResponse {
            access: AclAccess::Accept,
        })),
    }
}

// acl handler response
pub fn acl_target_pass() -> RouterHandlerAclResult{
    RouterHandlerAclResult {
        action: RouterHandlerAction::Pass,
        request: None,
        response: Some(Ok(AclHandlerResponse {
            access: AclAccess::Accept,
        })),
    }
}