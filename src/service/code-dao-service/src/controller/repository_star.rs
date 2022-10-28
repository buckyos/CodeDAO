use cyfs_base::*;
use cyfs_lib::*;

use async_std::sync::Arc;
use cyfs_git_base::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryDoStar {
    name: String,
    author_name: String,
    user_name: String,
}

/// # repository_do_star    
/// star 或者 unstar 仓库
pub async fn repository_do_star(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryDoStar = serde_json::from_str(&ctx.data).map_err(transform_err)?;

    if let Some(result) = ctx.check_space_proxy_request(&data.author_name).await? {
        return Ok(result);
    }

    let user_id = ctx.caller.to_string();

    let repository =
        RepositoryHelper::get_repository_object(&ctx.stack, &data.author_name, &data.name).await?;

    let key = RepositoryHelper::star_user_key(&data.author_name, &data.name, &user_id);
    let env = ctx.stack_env().await?;

    let result = env.get_by_path(&key).await?;
    if result.is_some() {
        // remove star
        let _r = env.remove_with_path(&key, None).await?;
        let root = env.commit().await;
        println!("remove commit: {:?}", root);
        return Ok(success(json!({"message": "unstar repository ok"})));
    }
    // add star
    let repository_star = RepositoryStar::create(
        ctx.caller,
        user_id,
        repository.id(),
        data.user_name,
        repository.author_name().to_string(),
        repository.name().to_string(),
    );

    let _r = put_object(&ctx.stack, &repository_star).await?;
    let object_id = repository_star.desc().calculate_id();

    let r = env.set_with_path(&key, &object_id, None, true).await?;
    println!("insert_with_key: {:?}", r);

    let root = env.commit().await;
    println!("commit: {:?}", root);

    Ok(success(json!({"message": "star repository ok"})))
}
