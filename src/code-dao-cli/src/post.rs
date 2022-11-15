use async_std::sync::Arc;
use cyfs_base::*;
//use cyfs_core::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use serde_json::*;
//use std::str::FromStr;

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
