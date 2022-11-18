use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;
//use std::thread;

pub struct Push {
    //cwd: String,
    repo: Arc<git2::Repository>,
    stack: Arc<SharedCyfsStack>,
    name: String,
    branch: String,
    ood: ObjectId,
    owner: ObjectId,
}

fn map_git_err(err: git2::Error) -> BuckyError {
    BuckyError::new(BuckyErrorCode::InternalError, err.to_string())
}

impl Push {
    pub fn new(
        repo: Arc<git2::Repository>,
        stack: Arc<SharedCyfsStack>,
        name: String,
        branch: String,
        ood: ObjectId,
        owner: ObjectId,
    ) -> Self {
        Self {
            repo,
            stack,
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
        info!("remote head oid {:?}", oid);

        let commits = self._commits().await?;

        // delta object
        // write tree

        Ok(())
    }

    pub async fn head_remote(&self) -> BuckyResult<Option<String>> {
        let head = self.repo.head().map_err(map_git_err)?;
        let branch = head.shorthand().expect("get branch failed");

        println!("{:?}", branch);
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
            info!("remote empty!");
            Ok(None)
        }
    }

    /// calculate commits
    async fn _commits(&self) -> BuckyResult<()> {
        let oid = self
            .repo
            .revparse_single(&self.branch)
            .map_err(map_git_err)?
            .id();
        info!("oid {}", oid.to_string());
        let mut revwalk = self.repo.revwalk().map_err(map_git_err)?;
        revwalk.push(oid).expect("ok");

        // TODO check if oid_end
        //let oid2 = git2::Oid::from_str("66bbc153590d2e9a56d238101527324b5491228b").expect("");
        //revwalk.hide(oid2).expect("ok");
        //info!("mark rev-list oid..oid2");

        for id in revwalk {
            let id = id.expect("id");
            let commit = self.repo.find_commit(id).expect("find commit");
            let commit_object = transform_commit(&commit, self.owner.clone());
            info!(
                "object id {}, {}",
                commit_object.id(),
                commit_object.object_id()
            );
            // commit tree 是肯定要写入rootstate的，其他的tree和blob可以先对比看情况
            let tree = commit.tree().expect("get commit tree failed");
            let tree_object = transform_tree(&tree, self.owner.clone());
            info!(
                "cyfs tree object {} {}",
                tree_object.id(),
                tree_object.tree_id()
            );
            // write commit, tree in rootstate
            put_object_target(&self.stack, &commit_object, Some(self.ood), None).await?;
            let env = self
                .stack
                .root_state_stub(Some(self.ood), Some(dec_id()))
                .create_path_op_env()
                .await?;
            let commit_path = rootstate_repo_commit2(&self.name, commit_object.object_id());
            env.set_with_path(
                commit_path,
                &commit_object.desc().calculate_id(),
                None,
                true,
            )
            .await?;
            info!("write commit into rootstate ok");

            put_object_target(&self.stack, &tree_object, Some(self.ood), None).await?;
            let tree_path = rootstate_repo_tree2(&self.name, tree_object.tree_id());
            env.set_with_path(&tree_path, &tree_object.desc().calculate_id(), None, true)
                .await?;
            info!("write tree into rootstate ok");
            env.commit().await?;
            //
            // TODO walk all
            // TODO write upload_ood fn

            let mut ct = 0;
            tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
                //info!("entry name {} ", entry.name().unwrap());
                //let kind = entry.kind().unwrap().to_string();
                //info!("entry kind {}", kind);
                if let Some(git2::ObjectType::Blob) = entry.kind() {
                    info!("blob object");
                    // upload
                }

                ct += 1;
                git2::TreeWalkResult::Ok
            })
            .unwrap();

            info!("tree walk {:?}", ct);

            info!("------");
            info!("");
        }

        // commits write into rootstate

        // cyfs commit object

        Ok(())
    }
}

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
    info!("parnets len {}", parents.len());

    let commit_object = Commit::create(
        owner,
        commit.id().to_string(),
        // TODO handler the multiple parents
        //commit.parent_id(0).unwrap().to_string(), // ?? PANIC
        parents,
        commit.tree_id().to_string(),
        commit.message().unwrap().to_string(),
        Some(author),
        Some(committer),
    );
    commit_object
}
