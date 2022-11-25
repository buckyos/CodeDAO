use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

type RequestRepositoryPushHead = RequestRepositoryDelete;

#[derive(Serialize, Deserialize)]
struct RequestRepositoryPush {
    // id: String,
    author_name: String,
    name: String,
    pack_file_id: String,
    // refs: String,
    branch: String,
    ref_hash: String,
    runtime_device_id: String,
    user_id: String,
    // dec_id: String,
}

#[derive(Serialize, Deserialize)]
struct RequestRepositoryPushTag {
    author_name: String,
    name: String,
    branch: String,
    ref_hash: String,
}

/// # repository_push_head    
/// git push head
pub async fn repository_push_head(
    ctx: Arc<PostContext>,
) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryPushHead = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    if let Some(result) = ctx.check_space_proxy_request(&space).await? {
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    if ctx.is_other_caller() {
        info!("others try to push, check member permission");
        let member_list = crate::member_list(&ctx.stack, &space, &name).await?;
        if member_list.len() == 0 {
            return Ok(failed("target repo are not permission granted"));
        }
        // check 是否有权限push
        let is_permission = member_list.iter().any(|member| {
            let owner_id = member["user_id"].as_str().unwrap().to_string();
            if owner_id != ctx.caller.to_string() {
                return false;
            }
            let role = RepositoryMemberRole::from_str(member["role"].as_str().unwrap())
                .expect("RepositoryMemberRole parse failed");
            // member["role"].as_str().unwrap();
            role.push_allow()
        });
        if !is_permission {
            return Ok(failed("target repo are not permission granted"));
        }
    }

    if repository.init() == 0 {
        return Ok(success(json!({"refs": []})));
    }

    let resp_refs = RepositoryBranch::read_refs(&ctx.stack, &space, &name).await?;
    Ok(success(json!({ "refs": resp_refs })))
}

/// # repository_push
/// git push
pub async fn repository_push(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryPush = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let mut local_path = cyfs_util::get_temp_path();
    local_path.push("cyfs-git");
    local_path.push(&data.pack_file_id);
    let runtime_device = DeviceId::from_str(&data.runtime_device_id).unwrap();
    let pack_file = ObjectId::from_str(&data.pack_file_id).unwrap();

    if !check_space_local(&ctx.stack, &space).await? {
        info!("other runtime push into");
        // A push to B
        // file:: A.runtime[publish_file] -> A.ood[create_task] -> B.ood[create_task]
        let _ = file_task(
            &ctx.stack,
            pack_file,
            local_path.clone(),
            runtime_device.clone(),
            None,
        )
        .await?;
        info!("download file to my ood ok, then proxy push request");
        let result = request_other_ood(&ctx.stack, &space, &ctx.route, &ctx.data).await?;
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    info!("get repository {:?}", repository.id());
    let _ = file_task(
        &ctx.stack,
        pack_file,
        local_path.clone(),
        runtime_device,
        Some(ctx.caller),
    )
    .await?;

    info!("repo push local_file path  {:?}", local_path);
    let repo_dir = repository.repo_dir();
    let _ = git_unbundle_and_unpack_objects(
        repo_dir.clone(),
        local_path.to_str().unwrap(),
        &data.branch,
        &data.ref_hash,
    )?;

    let repo_ref = RepositoryBranch::create(
        get_owner(&ctx.stack).await,
        space.clone(),
        name.clone(),
        data.branch.clone(),
        data.ref_hash,
    );
    repo_ref.insert_ref(&ctx.stack).await?;

    // update repository的init字段
    if repository.init() == 0 {
        Repository::update(
            repository,
            &ctx.stack,
            json!({
                "target": "init",
                "value": 1
            }),
        )
        .await?;
        info!("change repository column init 0 => 1 ");
    }

    let commit_path = RepositoryHelper::commit_object_map_path(&space, &name);
    info!("commit_path object id: {:?}", commit_path);

    // let branch = refs["branch"].as_str();
    let commits = git_commits(repo_dir.clone(), &data.branch)?;

    for commit in commits {
        let commit_object = Commit::create(
            ctx.caller,
            commit.object_id.clone(),
            vec![commit.parent, commit.parent2],
            commit.tree.clone(),
            commit.payload,
            Some(CommitSignature {
                name: commit.author.name,
                email: commit.author.email,
                when: commit.author.date,
            }),
            Some(CommitSignature {
                name: commit.committer.name,
                email: commit.committer.email,
                when: commit.committer.date,
            }),
            //serde_json::to_string(&commit.author).unwrap(), // 转换成json str
            //serde_json::to_string(&commit.committer).unwrap(),
            //commit.parent2,
        );
        let commit_object_id = commit_object.desc().object_id();
        println!("commit_obj object id: {:?}", commit_object_id);
        let env = ctx.stack_env().await?;
        let _r = env
            .set_with_key(
                &commit_path,
                &commit.object_id,
                &commit_object_id,
                None,
                true,
            )
            .await?;
        let root = env.commit().await;
        println!("new dec root is: {:?}", root);

        // put commit object
        put_object(&ctx.stack, &commit_object).await?;

        // info!("write tree object {:?}", commit.tree);
        // root_tree_object_loop_insert_map(&ctx.stack, ctx.caller, repo_dir.clone(), &commit.tree, &space, &name).await?;
    }

    info!("commit_path object map ok");
    Ok(success(json!({})))
}

/// # repository_push_tag
/// git push only tag
pub async fn repository_push_tag(ctx: Arc<PostContext>) -> BuckyResult<NONPostObjectInputResponse> {
    let data: RequestRepositoryPushTag = serde_json::from_str(&ctx.data).map_err(transform_err)?;
    let space = data.author_name;
    let name = data.name;
    let branch = data.branch;
    let ref_hash = data.ref_hash;

    if !check_space_local(&ctx.stack, &space).await? {
        let result = request_other_ood(&ctx.stack, &space, &ctx.route, &ctx.data).await?;
        return Ok(result);
    }

    let repository = RepositoryHelper::get_repository_object(&ctx.stack, &space, &name).await?;
    info!("get repository {:?}", repository.id());
    let repo_dir = repository.repo_dir();

    let repo_ref = RepositoryBranch::create(
        get_owner(&ctx.stack).await,
        space.clone(),
        name.clone(),
        branch.clone(),
        ref_hash.clone(),
    );
    repo_ref.insert_ref(&ctx.stack).await?;
    info!(
        "[{}/{}] update map ref {} {}",
        space, name, branch, ref_hash
    );

    git_update_ref(repo_dir, &branch, &ref_hash);
    info!(
        "[{}/{}] update repostiroy dir ref {} {}",
        space, name, branch, ref_hash
    );

    Ok(success(json!({})))
}

// struct CommitHelper<'a> {
//     env: &'a  SingleOpEnvStub,
// }

// impl <'a> CommitHelper <'a>{
//     pub fn new(env: &'a SingleOpEnvStub) -> CommitHelper {
//         CommitHelper { env: env }
//     }

//     pub async fn insert(&'a self, repo_dir: std::path::PathBuf, branch: &str, owner:ObjectId) ->BuckyResult<()> {
//         let commits = git_commits(repo_dir, branch)?;

//         for commit in commits {
//             let commit_object = Commit::create(
//                 owner,
//                 commit.object_id.clone(),
//                 commit.parent,
//                 commit.tree,
//                 commit.payload,
//                 commit.author,
//                 commit.committer,
//             );
//             let commit_object_id = commit_object.desc().object_id();
//             println!("commit_obj object id: {:?}", commit_object_id);
//             let r = env.insert_with_key(&commit.object_id, &commit_object_id).await?;
//             let root = env.commit().await;
//             println!("new dec root is: {:?}", root);
//         }

//         println!("commit_path object map ok");

//         Ok(())
//     }
// }
