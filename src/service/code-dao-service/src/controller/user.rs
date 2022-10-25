use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserInit {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserSetting {
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserGetByName {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestUserInfo {
    name: String,
    owner_id: String,
}

/// # user_init    
/// 用户信息初始化
pub async fn user_init(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserInit = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let key = ctx.caller.to_string();

    let env = ctx.stack_env().await?;
    let result = env.get_by_key(USER_LIST_PATH, &key).await?;
    if result.is_some() {
        println!("get_by_key: {:?}", result);
        return Ok(success(json!({"message": "already set"})));
    }

    let user_info = UserInfo::create(ctx.caller, data.name, data.email);
    info!(
        "send user object to service {}, {}",
        user_info.desc().object_id(),
        user_info.name()
    );
    post_special_object_service(&ctx.stack, &user_info, "user").await?;

    let r = put_object(&ctx.stack, &user_info).await?;
    info!("put user object local result: {:?}", r.result);

    let object_id = user_info.desc().calculate_id();
    let r = env
        .insert_with_key(USER_LIST_PATH, &key, &object_id)
        .await?;
    info!("target insert_with_key: {:?}", r);
    let _ = env.commit().await;

    // set to global current_space cache
    CURRET_SPACE
        .set(user_info.name().to_string())
        .map_err(|e| {
            error!("set_current_space: {:?}", e);
            BuckyError::new(
                BuckyErrorCode::InternalError,
                format!("set_current_space error:{}", e),
            )
        })?;

    info!("create user info ok, set to current space");

    Ok(success(json!({})))
}

/// # user_check_init    
/// 检查用户是初始化信息。 如果已经初始化就返回 用户信息
pub async fn user_check_init(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let user = UserHelper::get_current_user(&ctx.stack).await;
    if user.is_err() {
        if user.as_ref().err().unwrap().code() == BuckyErrorCode::NotFound {
            return Ok(success(json!({
                "userInit": false,
            })));
        }
        return Ok(failed(&format!(
            "user_check_init: {:?}",
            user.err().clone()
        )));
    }
    let user = user.unwrap();
    // let r = STACK_ACTION.get().unwrap().put_object(Some("user/init".to_string()), &user).await?;
    // info!("sync user object result: {:?}", r.result);

    Ok(success(json!({
        "userInit": true,
        "user": user.json(),
    })))
}

/// # user_setting    
/// 更改用户设置， email
pub async fn user_setting(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserSetting = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let key = ctx.caller.to_string();
    info!("full key: {}{}", USER_LIST_PATH, key);

    let env = ctx.stack_env().await?;
    // check key exist
    let result = env.get_by_key(USER_LIST_PATH, &key).await?;
    if result.is_none() {
        return Ok(failed("user no set infomation"));
    }

    let buf = get_object(&ctx.stack, result.unwrap()).await?;
    let prev_user = UserInfo::clone_from_slice(&buf)?;

    // TOFIX  创建对象的时间被改变了
    // create user
    let user_info = UserInfo::create(ctx.caller, prev_user.name().to_string(), data.email);
    let _r = put_object(&ctx.stack, &user_info).await?;
    let object_id = user_info.desc().calculate_id();

    let r = env
        .remove_with_key(USER_LIST_PATH, &key, Some(result.unwrap()))
        .await?;
    info!("remove_with_key: {:?}", r);
    {
        let r = env.insert_with_key(USER_LIST_PATH, &key, &object_id).await;
        info!("insert_with_key: {:?}", r);

        let root = env.commit().await;
        info!("new dec root is: {:?}", root);
    }

    return Ok(success(json!({"message": "OK"})));
}

/// # authors    
/// repository space select
/// 创建仓库的 space（当前用户+所在group）
pub async fn authors(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    // 当前用户
    let mut data: Vec<serde_json::Value> = Vec::new();
    let user = UserHelper::get_current_user(&ctx.stack).await?;
    data.push(user.json());

    // 所在group
    let env = ctx.stack_single_env().await?;
    let result = env.load_by_path(ORG_LIST_PATH).await;
    if result.is_ok() {
        if let Ok(ret) = env.list().await {
            for item in ret {
                info!("get org id: {:?}", item);
                let (_, id) = item.into_map_item();
                if let Ok(buf) = get_object(&ctx.stack, id).await {
                    let org = Organization::clone_from_slice(&buf)? as Organization;
                    data.push(json!({
                        "id": org.id(),
                        "name": org.name(),
                        "email": org.email(),
                        "type": "org",
                    }));
                } else {
                    error!("get org object failed, {:?}", id)
                }
            }
        }
    }

    Ok(success(json!({ "data": data })))
}

/// # user_list    
/// 用户列表
pub async fn user_list(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let (_, resp_body) = STACK_ACTION
        .get()
        .unwrap()
        .post_service("user/list", &ctx.data)
        .await?;
    Ok(success_proxy(resp_body))
}

/// # user_get_by_name    
/// 通过user/org 名字 获取信息
/// TODO 优化 sql
pub async fn user_get_by_name(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserGetByName = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    let env = ctx.stack_single_env().await?;
    env.load_by_path(USER_LIST_PATH).await?;

    loop {
        let ret = env.next(10).await.unwrap();
        if ret.len() == 0 {
            break;
        }

        for item in ret {
            let (_, id) = item.into_map_item();
            let user = UserHelper::user(&ctx.stack, id).await?;
            if data.name == user.name().to_string() {
                let owner_id = user.desc().owner().unwrap().to_string();
                return Ok(success(json!({
                    // "ownerId": owner_id,
                    "id": owner_id, // 这个字段有不一致的地方，有点尴尬
                    "name": user.name(),
                    "type": "user",
                    "message": "ok",
                })));
            }
        }
    }

    // Ok(success(json!({})))
    Ok(failed("target name not found"))
}

/// # user_info    
pub async fn user_info(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestUserInfo = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let is_local = check_space_local(&ctx.stack, &data.name).await?;
    if !is_local {
        return request_other_ood(&ctx.stack, &data.name, &ctx.route, &ctx.data).await;
    }

    let user = UserHelper::get_current_user(&ctx.stack).await;
    if user.is_err() {
        if user.as_ref().err().unwrap().code() == BuckyErrorCode::NotFound {
            return Ok(success(json!({
                "userInit": false,
            })));
        }
        return Ok(failed(&format!(
            "user_check_init: {:?}",
            user.err().clone()
        )));
    }

    Ok(success(json!({
        "userInit": true,
        "user": user.unwrap().json(),
    })))
}
