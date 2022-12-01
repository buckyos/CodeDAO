use async_recursion::async_recursion;
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
        self.branch().await?;
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
        let env = Arc::new(
            self.stack
                .root_state_stub(Some(self.ood_id), Some(dec_id()))
                .create_path_op_env()
                .await?,
        );
        let ret = env.list(tree_path).await?;
        for item in ret {
            let (tree_id, object_id) = item.into_map_item();
            info!("tree id {}", tree_id);

            let buf = get_object(&self.stack, object_id).await?;
            let tree = Tree::clone_from_slice(&buf)?;
            self._tree_item(Arc::clone(&env), tree).await?;
        }
        Ok(())
    }

    // write tree to cache
    // 这里需要通过递归来写tree
    // 假设仓库
    // - fileA
    // - src/fileB
    // - src/other/fileC
    // src 是root tree下的一个子项，但是先insert src会失败,
    // 失败的时候就递归先处理子项(src/other)
    #[async_recursion(?Send)]
    async fn _tree_item(&self, env: Arc<PathOpEnvStub>, tree: Tree) -> CodedaoResult<()> {
        let mut treebuilder = self.repo.treebuilder(None)?;
        let entries = tree.tree().to_owned();
        // tree 所有子项
        for entry in entries {
            info!("entry file name {}, {}", entry.file_name, entry.mode);
            let oid = git2::Oid::from_str(&entry.hash)?;

            // 同是tree类型，可能因为顺序问题 不存在.
            // 如果repo.find_tree成功，就不需要递归进去处理子项
            if "tree".eq(&entry.file_type) && self.repo.find_tree(oid.clone()).is_err() {
                info!("current sub tree no in cache db");
                let tree_path = rootstate_repo_tree2(&self.name, &oid.to_string());
                let object_id = env.get_by_path(tree_path).await?.unwrap();
                let buf = get_object(&self.stack, object_id).await?;
                let tree = Tree::clone_from_slice(&buf)?;
                self._tree_item(Arc::clone(&env), tree).await?;
            }
            let filemode: i32 = entry.mode.parse().unwrap();
            treebuilder.insert(entry.file_name, oid, filemode)?;
        }
        let tree_id_w = treebuilder.write()?;
        info!("check treebuilder write treeid {}", tree_id_w);
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

            // get cyfs commit object
            let buf = get_object(&self.stack, object_id).await?;
            let commit = Commit::clone_from_slice(&buf)?;

            let author = commit.author().to_owned().unwrap();
            let time = git2::Time::new(author.when, author.offset);
            let author2 = git2::Signature::new(&author.name, &author.email, &time)?;

            let committer = commit.committer().to_owned().unwrap();
            let time = git2::Time::new(committer.when, author.offset);
            let committer2 = git2::Signature::new(&committer.name, &committer.email, &time)?;

            let tree_id = git2::Oid::from_str(commit.tree_id())?;
            let tree = self.repo.find_tree(tree_id)?;

            let parents = commit.parents().to_owned();
            let oid = if parents.len() == 0 {
                self.repo
                    .commit(None, &author2, &committer2, commit.payload(), &tree, &[])?
            } else {
                // TODO parents how to pass?
                let id = parents[0].clone();
                let commit_id = git2::Oid::from_str(&id)?;
                let parent = self.repo.find_commit(commit_id)?;
                self.repo.commit(
                    None,
                    &author2,
                    &committer2,
                    commit.payload(),
                    &tree,
                    &[&parent],
                )?
            };
            info!("git2 gen commit oid {}", oid);
        }
        Ok(())
    }

    async fn branch(&self) -> CodedaoResult<()> {
        let branch_path = rootstate_repo_branchbase(&self.name);
        let env = self
            .stack
            .root_state_stub(Some(self.ood_id), Some(dec_id()))
            .create_single_op_env()
            .await?;
        env.load_by_path(branch_path).await?;
        let ret = env.list().await?;
        // TODO only read default.
        // So we need to a path name /<>/head, or in Repository object' desc
        for item in ret {
            let (branch, object_id) = item.into_map_item();
            let buf = get_object(&self.stack, object_id).await?;
            let branch_object = RepositoryBranch::clone_from_slice(&buf)?;
            let oid = git2::Oid::from_str(branch_object.ref_hash())?;
            let commit = self.repo.find_commit(oid)?;
            info!("branch name {}, current oid {}", branch, oid);
            let branch = self.repo.branch(branch_object.ref_name(), &commit, true)?;

            // TODO  move outside of this loop
            // set repo HEAD
            let name = format!("refs/heads/{}", branch.name()?.unwrap());
            self.repo.set_head(&name)?;
            info!("repository set HEAD:{} OK", name);
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
