use async_std::sync::Arc;
use async_std::task;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
//use std::thread;

pub struct Push {
    //cwd: String,
    repo: Arc<git2::Repository>,
    stack: Arc<SharedCyfsStack>,
    stack_util: Arc<StackUtil>,
    name: String,
    branch: String,
    ood: ObjectId,
    owner: ObjectId,
}

fn map_git_err(err: git2::Error) -> BuckyError {
    BuckyError::new(BuckyErrorCode::InternalError, err.to_string())
}

impl<'repo> Push {
    pub fn new(
        repo: Arc<git2::Repository>,
        stack: Arc<SharedCyfsStack>,
        stack_util: Arc<StackUtil>,
        name: String,
        branch: String,
        ood: ObjectId,
        owner: ObjectId,
    ) -> Self {
        Self {
            repo,
            stack,
            stack_util,
            name,
            branch,
            ood,
            owner,
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
        info!("Read remote: head ref oid {:?}", oid);

        let commits = self._commits().await?;

        // delta object
        // write tree

        Ok(())
    }

    pub async fn head_remote(&self) -> BuckyResult<Option<String>> {
        let head = self.repo.head().map_err(map_git_err)?;
        let branch = head.shorthand().expect("get branch failed");

        info!("current HEAD branch is {:?}", branch);
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        let branch_path = rootstate_repo_branch(&self.name, branch);
        //let ref_path

        if let Ok(Some(result)) = env.get_by_path(branch_path).await {
            info!("get remote branch oid {}", result.to_string());
            Ok(Some("".to_string()))
        } else {
            info!("remote/refs/{} empty!", branch);
            Ok(None)
        }
    }

    // TODO debug print a repository rootstate
    async fn debug(&self) {}

    /// calculate commits
    async fn _commits(&self) -> BuckyResult<()> {
        let oid = self
            .repo
            .revparse_single(&self.branch)
            .map_err(map_git_err)?
            .id();
        info!("Local HEAD branch's oid is {}", oid.to_string());
        let mut revwalk = self.repo.revwalk().map_err(map_git_err)?;
        revwalk.push(oid).expect("ok");

        // TODO check if oid_end
        //let oid2 = git2::Oid::from_str("66bbc153590d2e9a56d238101527324b5491228b").expect("");
        //revwalk.hide(oid2).expect("ok");
        //info!("mark rev-list oid..oid2");

        for id in revwalk {
            let id = id.expect("read commit oid failed");

            // write commit and tree into rootstate
            let commit = self.repo.find_commit(id).expect("find commit");
            self._write_commit(commit.clone()).await?;
            let tree = commit.tree().expect("get commit tree failed");
            self._write_tree(tree.clone()).await?;

            // TODO walk all
            let mut ct = 0;
            tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
                //info!("entry name {} ", entry.name().unwrap());
                //info!("entry kind {}", kind);
                if let Some(git2::ObjectType::Blob) = entry.kind() {
                    info!("blob object, {} {:?}", entry.id(), entry.name());
                    let oid = entry.id().to_string();
                    // TOFIX 如果blob objects被pack了?
                    let p = self
                        .repo
                        .path()
                        .join("objects")
                        .join(&oid[..2])
                        .join(&oid[2..]);

                    // block on await fn
                    // to upload a blob content, and write a cyfs blob object
                    let file_id = task::block_on(self.stack_util.upload(p))
                        .expect("upload blob content failed");
                    task::block_on(self._write_blob(oid.clone(), file_id.to_string()))
                        .expect("write blob failed");
                    info!("tree-walk upload and write blob[{}] success", oid);
                }
                // TODO tree object

                ct += 1;
                git2::TreeWalkResult::Ok
            })
            .unwrap();

            info!("repository commit tree walk count number: {:?}", ct);
            info!("------");
            info!("");
        }

        // commits write into rootstate
        // cyfs commit object

        Ok(())
    }

    // blob object
    async fn _write_blob(&self, blob_id: String, file_id: String) -> BuckyResult<()> {
        let blob_object = Blob::create(self.owner, blob_id.clone(), file_id);
        put_object_target(&self.stack, &blob_object, Some(self.ood), None).await?;
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        let blob_path = rootstate_repo_blob(&self.name, &blob_id);
        env.set_with_path(&blob_path, &blob_object.desc().calculate_id(), None, true)
            .await?;
        env.commit().await?;
        Ok(())
    }

    // put tree object 并写入rootstate
    async fn _write_tree(&self, tree: git2::Tree<'repo>) -> BuckyResult<()> {
        let tree_object = transform_tree(&tree, self.owner.clone());
        info!(
            "commit main tree, oid {}, cyfs id {}",
            tree_object.tree_id(),
            tree_object.id(),
        );
        // put object
        put_object_target(&self.stack, &tree_object, Some(self.ood), None).await?;
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        let tree_path = rootstate_repo_tree2(&self.name, tree_object.tree_id());
        env.set_with_path(&tree_path, &tree_object.desc().calculate_id(), None, true)
            .await?;
        env.commit().await?;
        Ok(())
    }

    // putobject 并写入rootstate
    async fn _write_commit(&self, commit: git2::Commit<'repo>) -> BuckyResult<()> {
        let commit_object = transform_commit(&commit, self.owner.clone());
        info!(
            "git commit oid {}, cyfs id {}",
            commit_object.object_id(),
            commit_object.id(),
        );
        put_object_target(&self.stack, &commit_object, Some(self.ood), None).await?;
        let commit_path = rootstate_repo_commit(&self.name, commit_object.object_id());
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        env.commit().await?;
        Ok(())
    }
}

// 转换cyfs object
fn transform_tree(tree: &git2::Tree, owner: cyfs_base::ObjectId) -> Tree {
    info!("tree id {}, {}", tree.id(), tree.len());
    let items = tree
        .iter()
        .map(|entry| {
            //info!("entry name {} ", entry.name().unwrap());
            //info!("entry kind {} ", entry.kind().unwrap());
            TreeItem {
                file_name: entry.name().unwrap().to_string(),
                file_type: entry.kind().unwrap().to_string(),
                mode: entry.filemode().to_string(),
                hash: entry.id().to_string(),
            }
        })
        .collect::<Vec<TreeItem>>();
    let tree_object = Tree::create(owner, tree.id().to_string(), items);
    tree_object
}

// 转换cyfs object
fn transform_commit(commit: &git2::Commit, owner: cyfs_base::ObjectId) -> Commit {
    let author = CommitSignature {
        name: commit.author().name().unwrap().to_string(),
        email: commit.author().email().unwrap().to_string(),
        when: commit.author().when().seconds().to_string(),
    };

    let committer = CommitSignature {
        name: commit.committer().name().unwrap().to_string(),
        email: commit.committer().email().unwrap().to_string(),
        when: commit.committer().when().seconds().to_string(),
    };

    let parents = commit
        .parents()
        .map(|parent| {
            info!("parent {}", parent.id());
            parent.id().to_string()
        })
        .collect::<Vec<String>>();

    if parents.len() == 0 {
        info!("current commit[{}] is init commit", commit.id());
    }
    //info!("parnets len {}", parents.len());
    let commit_object = Commit::create(
        owner,
        commit.id().to_string(),
        parents,
        commit.tree_id().to_string(),
        commit.message().unwrap().to_string(),
        Some(author),
        Some(committer),
    );
    commit_object
}
