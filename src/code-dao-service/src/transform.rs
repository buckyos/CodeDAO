use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
use std::path::PathBuf;

use std::str::FromStr;

pub struct Transform {
    name: String,
    path: String,
    ood_id: ObjectId,
    stack: Arc<SharedCyfsStack>,
    stack_util: Arc<StackUtil>,
    repo: git2::Repository,
}

impl Transform {
    pub fn new(
        name: String,
        path: String,
        stack: Arc<SharedCyfsStack>,
        ood_id: ObjectId,
        stack_util: Arc<StackUtil>,
    ) -> Self {
        let repo = Self::_repo(path.clone()).expect("init git2 repo");

        Self {
            repo,
            name,
            path,
            stack,
            ood_id,
            stack_util,
        }
    }

    fn _repo(path: String) -> CodedaoResult<git2::Repository> {
        let p = PathBuf::from(path);
        let repo = if let Ok(r) = git2::Repository::open_bare(p.clone()) {
            r
        } else {
            git2::Repository::init_bare(p)?
        };

        Ok(repo)
    }

    async fn rebuild(&self) -> CodedaoResult<()> {
        self.blob().await?;
        self.tree().await?;
        self.commit().await?;
        Ok(())
    }

    async fn blob(&self) -> CodedaoResult<()> {
        let p = PathBuf::from(self.path.clone());

        let env = self
            .stack
            .root_state_stub(Some(self.ood_id), Some(dec_id()))
            .create_single_op_env()
            .await?;
        let blob_path = rootstate_repo_blobbase(&self.name);

        info!("blob path {}", blob_path);
        env.load_by_path(blob_path).await?;
        let ret = env.list().await?;
        for item in ret {
            let (blob_id, object_id) = item.into_map_item();
            info!("blob id {}", blob_id);
            let blob_dir = p.join("objects").join(&blob_id[..2]);
            let blob_path = blob_dir.join(&blob_id[2..]);

            if blob_path.exists() {
                info!("blob object[{}] exist in repository data", blob_id);
                continue;
            }

            let buf = get_object(&self.stack, object_id).await?;
            let blob = Blob::clone_from_slice(&buf)?;
            info!("blob {} {}", blob.blob_id(), blob.file_id());

            std::fs::create_dir(blob_dir).expect("create objects dir failed");
            let file_id = ObjectId::from_str(blob.file_id()).unwrap();
            self.stack_util.download(file_id, blob_path).await?;
        }
        Ok(())
    }

    // rebuild TREE object
    async fn tree(&self) -> CodedaoResult<()> {
        let tree_path = rootstate_repo_treebase(&self.name);
        let env = self
            .stack
            .root_state_stub(Some(self.ood_id), Some(dec_id()))
            .create_single_op_env()
            .await?;
        env.load_by_path(tree_path).await?;
        let ret = env.list().await?;
        for item in ret {
            let (tree_id, object_id) = item.into_map_item();
            info!("tree id {}", tree_id);

            let buf = get_object(&self.stack, object_id).await?;
            let tree = Tree::clone_from_slice(&buf)?;
            // tree.tree_id()
            let entries = tree.tree().to_owned();
            for entry in entries {
                info!("entry file name {}, {}", entry.file_name, entry.mode);

                // write to local
                let mut treebuilder = self.repo.treebuilder(None)?;
                let oid = git2::Oid::from_str(&entry.hash)?;
                let filemode: i32 = entry.mode.parse().unwrap();
                treebuilder.insert(entry.file_name, oid, filemode)?;
                let tree_id_w = treebuilder.write()?;
                info!("check treebuilder write treeid {}", tree_id_w);
                if tree_id_w.to_string() != tree_id {
                    error!("tree builder gen treeid no same with rootstate saved");
                }
            }
        }
        Ok(())
    }
    // rebuild Commit object
    async fn commit(&self) -> CodedaoResult<()> {
        let commit_path = rootstate_repo_commitbase(&self.name);
        let env = self
            .stack
            .root_state_stub(Some(self.ood_id), Some(dec_id()))
            .create_single_op_env()
            .await?;
        env.load_by_path(commit_path).await?;
        let ret = env.list().await?;

        for item in ret {
            let (commit_id, object_id) = item.into_map_item();
            info!("commit id {}", commit_id);

            // let buf = get_object(&self.stack, object_id).await?;
            // let commit = Commit::clone_from_slice(&buf)?;
        }
        Ok(())
    }
}

pub async fn transform_test(stack: Arc<SharedCyfsStack>) -> CodedaoResult<()> {
    let name = "2022_1110";
    let owner = owner(&stack);
    let name = format!("{}/{}", owner.to_string(), name);
    let ood = get_ood_device(&stack).await;

    // TODO move new
    let stack_util = Arc::new(StackUtil::new(
        Arc::clone(&stack),
        owner.clone(),
        ood.clone(),
    ));

    let t = Transform::new(
        name,
        "/home/aa/test/trans_test".to_string(),
        Arc::clone(&stack),
        ood,
        Arc::clone(&stack_util),
    );
    // build .git cached
    //
    // 再commit
    info!("start env");
    t.rebuild().await.expect("rebuild failed");

    Ok(())
}
