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
    pub fn index(&self) -> CodedaoResult<String> {
        let obj = self
            .repo
            .head()?
            .resolve()?
            .peel(git2::ObjectType::Commit)?;
        let commit = obj.into_commit().expect("into commit failed");
        let id = commit.id().to_string();
        Ok(id)
    }

    pub async fn push(&self) -> CodedaoResult<()> {
        let head = self.repo.head()?;
        let branch = head.shorthand().expect("get branch name failed");

        let oid = self.head_remote(branch).await?;
        info!("Read remote: head ref oid {:?}", oid);

        // check remote HEAD oid
        //
        if oid.is_none() {
            info!("remote[{}] empty", branch);
            self.update_remote_branch().await?;
            // TODO set a default branch
        }

        // handle commits
        let commits = self._commits().await?;
        // TODO delta object

        Ok(())
    }

    pub async fn head_remote(&self, branch: &str) -> CodedaoResult<Option<String>> {
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

    async fn update_remote_branch(&self) -> CodedaoResult<()> {
        info!("update remote branch[{}](ref) value", self.branch);
        let head = self.repo.head()?;
        let branch = head.shorthand().expect("get branch name failed");
        let oid = self.repo.revparse_single(branch)?.id().to_string();
        let mut name = self.name.split("/").into_iter();
        let space = name.next().unwrap().to_owned();
        let repo_name = name.next().unwrap().to_owned();

        let branch_object = RepositoryBranch::create(
            self.owner,
            space,
            repo_name,
            branch.to_string(),
            oid.clone(),
        );
        put_object_target(&self.stack, &branch_object, Some(self.ood), None).await?;
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        let branch_path = rootstate_repo_branch(&self.name, branch);
        env.set_with_path(
            &branch_path,
            &branch_object.desc().calculate_id(),
            None,
            true,
        )
        .await?;
        env.commit().await?;
        info!("wirte branch:{} {} to rootstate ok", branch, oid);
        Ok(())
    }

    // TODO debug print a repository rootstate, like fd
    async fn debug(&self) {
        // print
    }

    /// calculate commits
    async fn _commits(&self) -> CodedaoResult<()> {
        let oid = self.repo.revparse_single(&self.branch)?.id();
        info!("Local HEAD branch's oid is {}", oid.to_string());
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(oid)?;

        // TODO check if oid_end
        //let oid2 = git2::Oid::from_str("66bbc153590d2e9a56d238101527324b5491228b").expect("");
        //revwalk.hide(oid2).expect("ok");
        //info!("mark rev-list oid..oid2");

        for id in revwalk {
            let id = id?;

            // write commit and tree into rootstate
            let commit = self.repo.find_commit(id)?;
            self._write_commit(commit.clone()).await?;
            let tree = commit.tree()?;
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
            })?;

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
        let env = self
            .stack
            .root_state_stub(Some(self.ood), Some(dec_id()))
            .create_path_op_env()
            .await?;
        let commit_path = rootstate_repo_commit(&self.name, commit_object.object_id());
        env.set_with_path(
            commit_path,
            &commit_object.desc().calculate_id(),
            None,
            true,
        )
        .await?;
        env.commit().await?;
        Ok(())
    }

    async fn _write_ref(&self) -> BuckyResult<()> {
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
    let author = commit.author();
    let author = CommitSignature {
        name: author.name().unwrap().to_string(),
        email: author.email().unwrap().to_string(),
        when: author.when().seconds(),
        offset: author.when().offset_minutes(),
        sign: author.when().sign().to_string(),
    };
    let committer = commit.committer();
    let committer = CommitSignature {
        name: committer.name().unwrap().to_string(),
        email: committer.email().unwrap().to_string(),
        when: committer.when().seconds(),
        offset: committer.when().offset_minutes(),
        sign: committer.when().sign().to_string(),
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
