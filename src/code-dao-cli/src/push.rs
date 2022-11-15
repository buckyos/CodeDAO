use async_std::sync::Arc;
use cyfs_base::{BuckyError, BuckyErrorCode, BuckyResult};
use cyfs_git_base::*;
use cyfs_lib::SharedCyfsStack;
use log::*;

pub struct Push {
    //cwd: String,
    repo: Arc<git2::Repository>,
    stack: Arc<SharedCyfsStack>,
    name: String,
}

fn map_git_err(err: git2::Error) -> BuckyError {
    BuckyError::new(BuckyErrorCode::InternalError, err.to_string())
}

impl Push {
    pub fn new(repo: Arc<git2::Repository>, stack: Arc<SharedCyfsStack>, name: String) -> Self {
        Self {
            //      c
            repo,
            stack,
            name,
        }
    }
    pub fn index(&self) -> Result<String, git2::Error> {
        let obj = self
            .repo
            .head()?
            .resolve()?
            .peel(git2::ObjectType::Commit)?;
        let commit = obj
            .into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
        let id = commit.id().to_string();
        Ok(id)
    }

    pub async fn push(&self) -> BuckyResult<()> {
        let oid = self.head_remote().await?;

        // delta object
        // write tree

        Ok(())
    }

    pub async fn head_remote(&self) -> BuckyResult<Option<String>> {
        let head = self.repo.head().map_err(map_git_err)?;
        let branch = head.shorthand().expect("get branch failed");

        println!("{:?}", branch);
        let ood = Some(get_ood_device(&self.stack).await);
        let env = self
            .stack
            .root_state_stub(ood, Some(dec_id()))
            .create_path_op_env()
            .await?;
        let branch_path = rootstate_repo_branch(&self.name, branch);
        //let ref_path

        if let Ok(Some(result)) = env.get_by_path(branch_path).await {
            info!("get remote branch oid {}", result.to_string());
            Ok(Some("".to_string()))
        } else {
            info!("remote empty!");
            Ok(None)
        }
    }
}
