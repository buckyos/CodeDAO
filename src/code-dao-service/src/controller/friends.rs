
use cyfs_lib::*;
use cyfs_base::*;
//use cyfs_core::*;
use async_std::sync::Arc;
// use serde::{Deserialize, Serialize};
use serde_json::json;
use cyfs_git_base::*;
//use log::*;


/// # friends    
/// 
pub async fn friends(_ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {

    //let friend_list_object = FriendList::create(get_owner(&ctx.stack).await, false);
    //let friend_list_id = friend_list_object.desc().calculate_id();
    //info!("get friend_list object id {:?}", friend_list_id);

    // stack.non_service().
    //let result = get_object(&ctx.stack, friend_list_id).await;
    //if result.is_err() {
      //  error!("get friend list error ");
        //return Ok(success(json!({"data": []})))
    //}
    //let buf = result.unwrap();
    //let friend_list = FriendList::clone_from_slice(&buf)? as FriendList;

    let data: Vec<serde_json::Value> = Vec::new();

    //for item in friend_list.friend_list() {
      //  data.push(json!({
        //    "id": item.0.to_string(),
        //}));
    //}

    Ok(success(json!({"data": data})))
}
