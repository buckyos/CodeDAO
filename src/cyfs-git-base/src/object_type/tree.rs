use crate::*;
use async_trait::async_trait;
use cyfs_base::*;
use cyfs_lib::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::TreeItem)]
pub struct TreeItem {
    pub mode: String,
    pub hash: String,
    pub file_name: String,
    pub file_type: String,
}

#[derive(Clone, Default, ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[cyfs_protobuf_type(crate::object_type::proto::cyfs_git::TreeDescContent)]
pub struct TreeDescContent {
    tree_id: String,
    tree: Vec<TreeItem>,
}

impl DescContent for TreeDescContent {
    fn obj_type() -> u16 {
        CustomObjType::Tree as u16
    }
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
    type OwnerType = Option<ObjectId>;
    type AreaType = SubDescNone;
    type AuthorType = SubDescNone;
    type PublicKeyType = SubDescNone;
}

#[derive(Clone, Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
pub struct TreeBodyContent {}

impl BodyContent for TreeBodyContent {
    fn format(&self) -> u8 {
        OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF
    }
}

type TreeType = NamedObjType<TreeDescContent, TreeBodyContent>;
type TreeBuilder = NamedObjectBuilder<TreeDescContent, TreeBodyContent>;
// type TreeDesc = NamedObjectDesc<TreeDescContent>;

pub type TreeId = NamedObjectId<TreeType>;
pub type Tree = NamedObjectBase<TreeType>;

pub trait TreeObject {
    fn create(owner: ObjectId, tree_id: String, tree: Vec<TreeItem>) -> Self;
    fn id(&self) -> String;
    fn date(&self) -> u64;
    fn tree_id(&self) -> &String;
    fn tree(&self) -> &Vec<TreeItem>;
}

impl TreeObject for Tree {
    fn create(owner: ObjectId, tree_id: String, tree: Vec<TreeItem>) -> Self {
        let desc = TreeDescContent { tree_id, tree };
        let body = TreeBodyContent {};

        TreeBuilder::new(desc, body)
            .owner(owner)
            .dec_id(dec_id())
            .option_create_time(None)
            .build()
    }
    fn id(&self) -> String {
        self.desc().calculate_id().to_string()
    }
    fn date(&self) -> u64 {
        bucky_time_to_js_time(self.desc().create_time())
    }
    fn tree_id(&self) -> &String {
        &self.desc().content().tree_id
    }
    fn tree(&self) -> &Vec<TreeItem> {
        &self.desc().content().tree
    }
}

#[async_trait]
pub trait TreeUtil {
    async fn insert_tree(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
    ) -> BuckyResult<()>;
    async fn get_tree_object(
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
        tree_oid: &str,
    ) -> BuckyResult<Tree>;
    async fn get_tree_from_path(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
        sub_dir: &str,
    ) -> BuckyResult<Tree>;
    fn read_tree_file_map(&self) -> BuckyResult<HashMap<String, (String, String)>>;
    fn get_tree_from_git(
        owner: ObjectId,
        space: &str,
        repo_name: &str,
        tree_oid: &str,
    ) -> BuckyResult<Tree>;
}

#[async_trait]
impl TreeUtil for Tree {
    async fn insert_tree(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
    ) -> BuckyResult<()> {
        let id = self.desc().calculate_id();
        let _r = put_object(stack, self).await?;
        let env = create_path_op_env(stack).await?;
        let full_path = rootstate_repo_tree(space, repo_name, self.tree_id());
        let result = env.get_by_path(full_path.clone()).await?;
        if result.is_some() {
            info!(
                "repository tree object[{}] alread in objectmap",
                self.tree_id()
            );
            return Ok(());
        }
        let _r = env
            .set_with_path(full_path.clone(), &id, None, true)
            .await?;
        let _r = env.commit().await?;
        info!(
            "insert repository tree object ok {}: {}",
            full_path,
            self.tree_id()
        );
        Ok(())
    }

    // quick get object
    async fn get_tree_object(
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
        tree_oid: &str,
    ) -> BuckyResult<Tree> {
        let env = create_path_op_env(stack).await?;
        let full_path = rootstate_repo_tree(space, repo_name, &tree_oid);
        let object_id = env.get_by_path(full_path).await?;
        if object_id.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::NotFound,
                "not found tree object id",
            ));
        }
        let object_id = object_id.unwrap();
        let buf = get_object(stack, object_id).await?;
        let tree = Tree::clone_from_slice(&buf)?;
        Ok(tree)
    }

    // recurse
    async fn get_tree_from_path(
        &self,
        stack: &Arc<SharedCyfsStack>,
        space: &str,
        repo_name: &str,
        sub_dir: &str,
    ) -> BuckyResult<Tree> {
        let target = self
            .tree()
            .into_iter()
            .find(|x| sub_dir.to_string() == x.file_name);
        if target.is_none() {
            return Err(BuckyError::new(
                BuckyErrorCode::NotFound,
                "not found target dir's tree",
            ));
        }
        let target = target.unwrap();
        let current_tree_id = target.hash.clone();
        // info!("target sub tree oid {} ", target.hash);
        let tree = Tree::get_tree_object(stack, space, repo_name, &current_tree_id).await;
        if tree.is_err() {
            let owner = get_owner(stack).await;
            let tree_object = Tree::get_tree_from_git(owner, space, repo_name, &current_tree_id)?;
            return Ok(tree_object);
        }

        let tree = tree.unwrap();
        Ok(tree)
    }

    fn read_tree_file_map(&self) -> BuckyResult<HashMap<String, (String, String)>> {
        Ok(self
            .tree()
            .into_iter()
            .map(|tree_item| {
                // info!("main_file_map {}", tree_item.file_name);
                (
                    tree_item.file_name.clone(),
                    (tree_item.hash.clone(), tree_item.file_type.clone()),
                )
            })
            .collect())
    }

    fn get_tree_from_git(
        owner: ObjectId,
        space: &str,
        repo_name: &str,
        tree_oid: &str,
    ) -> BuckyResult<Tree> {
        let repo_dir = RepositoryHelper::repo_dir(space, repo_name);
        // 封装这段。 和下面有点重复
        let tree_result = git_exec_base(repo_dir, ["cat-file", "-p", tree_oid])?;
        let tree_array: Vec<GitLsTreeResultTreeData> = tree_result
            .lines()
            .map(|line| git_ls_object_parse(line).unwrap())
            .collect();
        let mut tree: Vec<TreeItem> = Vec::new();
        for item in tree_array {
            tree.push(TreeItem {
                mode: item.file_mode,
                hash: item.file_hash,
                file_name: item.file_name,
                file_type: item.git_object_type,
            })
        }
        let tree_object = Tree::create(owner, tree_oid.to_string(), tree);
        Ok(tree_object)
    }
}

pub async fn commits_root_tree_object_loop_insert_map(
    stack: &Arc<SharedCyfsStack>,
    owner: ObjectId,
    repo_dir: PathBuf,
    top_commit_id: &str,
    space: &str,
    repo_name: &str,
) -> BuckyResult<()> {
    let commit = Arc::new(git_read_commit_object(repo_dir.clone(), top_commit_id)?);
    let mut commit_count = 1;
    let mut loop_stack: Vec<Arc<GitCommit>> = vec![commit];
    loop {
        if loop_stack.len() == 0 {
            break;
        }

        let current_commit = loop_stack.remove(0);
        if current_commit.parent != "" {
            loop_stack.push(Arc::new(git_read_commit_object(
                repo_dir.clone(),
                &current_commit.parent,
            )?));
        }
        if current_commit.parent2 != "" {
            loop_stack.push(Arc::new(git_read_commit_object(
                repo_dir.clone(),
                &current_commit.parent2,
            )?));
        }

        // info!("commit {:?},  poid {:?}", commit.object_id, commit.parent);
        root_tree_object_loop_insert_map(
            stack,
            owner,
            repo_dir.clone(),
            &current_commit.tree,
            space,
            repo_name,
        )
        .await?;
        commit_count += 1
    }

    info!("count {}", commit_count);

    Ok(())
}

// root_tree_object_loop_insert_map
// TODO loop all commit
pub async fn root_tree_object_loop_insert_map(
    stack: &Arc<SharedCyfsStack>,
    owner: ObjectId,
    repo_dir: PathBuf,
    root_tree_id: &str,
    space: &str,
    repo_name: &str,
) -> BuckyResult<()> {
    let mut tree_id_array: Vec<String> = vec![];
    tree_id_array.push(root_tree_id.to_string());

    loop {
        let current_tree_id = tree_id_array.pop();
        if current_tree_id.is_none() {
            break;
        }
        let current_tree_id = current_tree_id.unwrap();
        let tree_result = git_exec_base(repo_dir.clone(), ["cat-file", "-p", &current_tree_id])?;
        let tree_array: Vec<GitLsTreeResultTreeData> = tree_result
            .lines()
            .map(|line| git_ls_object_parse(line).unwrap())
            .collect();

        let mut tree: Vec<TreeItem> = Vec::new();
        for item in tree_array {
            if item.git_object_type == "tree" {
                info!("find tree id in tree list {}", item.file_hash);
                tree_id_array.push(item.file_hash.clone());
            }
            tree.push(TreeItem {
                mode: item.file_mode,
                hash: item.file_hash,
                file_name: item.file_name,
                file_type: item.git_object_type,
            })
        }
        let tree_object = Tree::create(owner, current_tree_id, tree);
        let _ = tree_object.insert_tree(stack, space, repo_name).await?;
        info!("tree len {}", tree_id_array.len());
    }
    info!("mount tree object in objectmap all ok");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeFileMessage2 {
    pub file_type: String,
    pub hash: String,
    pub message: String,
    pub date: String,
    pub author: String,
}

pub struct CommitLoopStack {
    index: usize,
    loop_stack: Vec<String>,
    commits: Arc<HashMap<String, Arc<GitCommit>>>,
}

impl CommitLoopStack {
    pub fn new(commits: Arc<HashMap<String, Arc<GitCommit>>>) -> Self {
        Self {
            index: 0,
            loop_stack: vec![],
            commits,
        }
    }
    pub fn add(&mut self, commit_oid: String) {
        if !self.loop_stack.contains(&commit_oid) {
            self.loop_stack.push(commit_oid);
        }
    }
    pub fn shift(&mut self) -> Arc<GitCommit> {
        let oid = self.loop_stack.get(self.index).unwrap();
        self.index = self.index + 1;
        let c = self.commits.get(oid).unwrap();
        Arc::clone(c)
    }

    pub fn drain(&mut self) -> bool {
        let len = self.loop_stack.len();
        self.index + 1 > len
    }

    pub fn update(&mut self, commit: Arc<GitCommit>) {
        if &commit.parent != "" {
            self.add(commit.parent.clone())
        }
        if &commit.parent2 != "" {
            self.add(commit.parent2.clone())
        }
    }
}

pub struct TreeFileHelper {
    stack: Arc<SharedCyfsStack>,
    loop_stack: CommitLoopStack,
    init_file_map: HashMap<String, (String, String)>,
    result_main_file_map: HashMap<String, TreeFileMessage2>,
    space: String,
    repo_name: String,
    lastest_oid: String,
    // branch: String,
    sub_dir: String,
    commits: Arc<HashMap<String, Arc<GitCommit>>>,
    merged_commit_mark: HashMap<String, usize>,
    mark_commit_relationship: HashMap<String, Arc<Mutex<Vec<String>>>>,
}

// #[async_trait]
impl TreeFileHelper {
    pub async fn new(
        stack: Arc<SharedCyfsStack>,
        space: String,
        repo_name: String,
        lastest_oid: String,
        branch: String,
        sub_dir: String,
    ) -> BuckyResult<Self> {
        let commits = Arc::new(commits_of_branch(&stack, &space, &repo_name, &branch).await?);
        Ok(Self {
            stack,
            loop_stack: CommitLoopStack::new(Arc::clone(&commits)),
            init_file_map: HashMap::new(),
            result_main_file_map: HashMap::new(),
            lastest_oid,
            sub_dir,
            space,
            repo_name,
            // branch,
            commits,
            merged_commit_mark: HashMap::new(),
            mark_commit_relationship: HashMap::new(),
        })
    }
    // mark merge & mark child relationship
    pub async fn prepare(&mut self) -> BuckyResult<()> {
        info!("tree file prepare before start");
        let commit = self.commits.get(&self.lastest_oid).unwrap();
        self.init_file_map = self.target_tree_file(Arc::clone(commit)).await?;

        // let commits = self.commits.clone();
        // self.commits.iter().for_each(|(_, commit)| {
        // });
        let commits = Arc::clone(&self.commits);
        // mark
        for (_, commit) in commits.iter() {
            if commit.parent != "".to_string() {
                let parent_oid = commit.parent.clone();
                let oid = commit.object_id.clone();
                self.relationship(parent_oid, oid);
            }
            if commit.parent2 != "".to_string() {
                let parent_oid = commit.parent2.clone();
                let oid = commit.object_id.clone();
                self.relationship(parent_oid, oid);
            }
            if commit.parent != "".to_string() && commit.parent2 != "".to_string() {
                self.merged_commit_mark.insert(commit.object_id.clone(), 1);
            }
        }
        info!("merge point len{}", self.merged_commit_mark.len());
        info!("tree file prepare ok");
        Ok(())
    }

    // 快速的去递归子树(子目录)
    // pub async fn target_tree(&self, commit: Rc<Commit>) -> BuckyResult<Rc<HashMap<String, String>>> {
    pub async fn target_tree_file(
        &self,
        commit: Arc<GitCommit>,
    ) -> BuckyResult<HashMap<String, (String, String)>> {
        let mut it = std::path::Path::new(&self.sub_dir).iter();

        let mut tree = {
            let result =
                Tree::get_tree_object(&self.stack, &self.space, &self.repo_name, &commit.tree)
                    .await;
            if result.is_ok() {
                result.unwrap()
            } else {
                let owner = get_owner(&self.stack).await;
                let tree_object =
                    Tree::get_tree_from_git(owner, &self.space, &self.repo_name, &commit.tree)?;
                tree_object
            }
        };

        // let mut tree = commit.tree(&self.stack, &self.space, &self.repo_name).await?;
        while let Some(target_path_step) = it.next() {
            let target_path_step = target_path_step.to_str().unwrap();
            info!("path step {}", target_path_step);
            tree = tree
                .get_tree_from_path(&self.stack, &self.space, &self.repo_name, target_path_step)
                .await?;
        }
        // let tree_file_map = Rc::new(tree.read_tree_file_map()?);
        // Ok(tree_file_map)
        let tree_file_map = tree.read_tree_file_map()?;
        // clone 优化?
        Ok(tree_file_map.clone())
    }

    // match handler
    pub fn file_match<S: Into<String>>(
        &mut self,
        key: S,
        commit: Arc<GitCommit>,
        file_type: String,
    ) {
        let key = key.into();
        self.result_main_file_map.insert(
            key.clone(),
            TreeFileMessage2 {
                file_type: file_type,
                hash: commit.object_id.clone(),
                message: commit.payload.clone(),
                author: serde_json::to_string(&commit.author).unwrap(),
                date: serde_json::to_string(&commit.committer).unwrap(),
            },
        );
        self.init_file_map.remove(&key);
    }

    // mark commit relationship
    // 类似把单向链表变成双向链表
    fn relationship(&mut self, key: String, children_oid: String) {
        if self.mark_commit_relationship.contains_key(&key) {
            // update
            let value = self.mark_commit_relationship.get(&key).unwrap();
            let mut lock = value.try_lock().unwrap();
            if !lock.contains(&children_oid) {
                info!("lock.contains");
                lock.push(children_oid);
            }
            drop(lock);
            info!("mark_commit_relationship unlock");
        } else {
            // new
            self.mark_commit_relationship
                .insert(key.to_string(), Arc::new(Mutex::new(vec![children_oid])));
        }
    }

    // get target child from relationship's vec<>
    fn child(&mut self, oid: &str) -> Arc<GitCommit> {
        let child_commit_vec = self.mark_commit_relationship.get(oid).unwrap();
        let child_commit_vec = child_commit_vec.lock().unwrap();
        let c = {
            if child_commit_vec.len() > 1 {
                info!("try get child child_oid {:?}", oid);
                let child_oid = child_commit_vec
                    .clone()
                    .into_iter()
                    .find(|x| !self.merged_commit_mark.contains_key(x))
                    .unwrap();
                let c = self.commits.get(&child_oid).unwrap();
                c
            } else {
                let child_oid = &child_commit_vec[0];
                // .unwrap()[0];
                let c = self.commits.get(child_oid).unwrap();
                c
            }
        };

        Arc::clone(c)
    }

    // start main loop
    pub async fn start(&mut self) -> BuckyResult<HashMap<String, TreeFileMessage2>> {
        self.loop_stack.add(self.lastest_oid.clone());

        info!("start tree file main loop ---");
        loop {
            if self.loop_stack.drain() || self.init_file_map.len() == 0 {
                break;
            }
            let commit = self.loop_stack.shift();
            let is_bottom = &commit.parent == ""; // 可能有多个的。
            let current_commit_oid = commit.object_id.clone();
            info!("commit id {} ", current_commit_oid);
            self.loop_stack.update(Arc::clone(&commit));
            // info!("stack len {}", self.len());
            let file_map = self.target_tree_file(Arc::clone(&commit)).await;
            if file_map.is_err() {
                info!(
                    "no tree file , set child commit oid and info [{}]",
                    current_commit_oid
                );
                for (file_name, value) in self.init_file_map.clone().into_iter() {
                    let target_commit = self.child(&current_commit_oid);
                    self.file_match(file_name, target_commit, value.1.clone());
                }
                // return Err(BuckyError::new(BuckyErrorCode::NotFound, format!("get tree file error {:?}", file_map.err())))
                break;
            }
            let file_map = file_map.unwrap();

            for (file_name, value) in self.init_file_map.clone().into_iter() {
                if file_map.get(&file_name).is_none() {
                    // new file. write result map
                    info!(
                        "tree no file [{}] , set child commit oid [{}]",
                        file_name, current_commit_oid
                    );
                    let target_commit = self.child(&current_commit_oid);
                    self.file_match(file_name, target_commit, value.1.clone());
                } else {
                    // arr:[hash,file_type]
                    let prev_file_hash = value.0;
                    let comparison = file_map.get(&file_name).unwrap();
                    let current_file_hash = comparison.0.to_string();

                    // check
                    if prev_file_hash != current_file_hash {
                        info!(
                            "file[{}] in tree and modify  , set child commit oid [{}]",
                            file_name, current_commit_oid
                        );
                        let target_commit = self.child(&current_commit_oid);
                        self.file_match(file_name, target_commit, value.1.clone());
                    } else if is_bottom && prev_file_hash == current_file_hash {
                        info!(
                            "no file[{}] and init commit  , set current commit oid [{}]",
                            file_name, current_commit_oid
                        );
                        self.file_match(file_name, Arc::clone(&commit), value.1.clone());
                    }
                }
            }
        } //

        // info!("len of normal mark  {}", self.init_file_map.len());
        Ok(self.result_main_file_map.clone()) // rc? cell
    }
}

// test
#[cfg(test)]
mod main_tests {
    use super::*;

    async fn test_base() -> BuckyResult<Arc<SharedCyfsStack>> {
        cyfs_debug::CyfsLoggerBuilder::new_app("cyfs-git-test")
            .level("info")
            .console("info")
            .enable_bdt(Some("error"), Some("error"))
            .module("cyfs_lib", Some("error"), Some("error"))
            .build()
            .unwrap()
            .start();
        ConfigManager::new_oncecell_with_content(
            r#"
[main]
channel="dev-test"
deploy_owner_id="5r4MYfFQz9iEzjwHUpc79CwrvXqsh7xUzynkiTUEckxB"
public_service_ood="5aSixgM1oBicrsUdS3nyKM1MA9AgiMEE2y2qFQ3jTTYB""#,
        );
        info!("dec {}", dec_id());
        let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
        // let stack = Arc::new(SharedCyfsStack::open_runtime(Some(dec_id())).await.unwrap());
        stack.wait_online(None).await?;
        Ok(stack)
    }

    fn last_commit_oid(path: PathBuf) -> BuckyResult<String> {
        let refs_hash = git_exec_base(path, ["show-ref", "-s", "refs/heads/master"])?;
        let oid = refs_hash.trim();
        Ok(oid.to_string())
    }

    const SPACE: &'static str = "sunxinle001";
    const REPO_NAME: &'static str = "20220902newtest";

    #[async_std::test]
    async fn repository_read_and_write_tree_object() -> BuckyResult<()> {
        let stack = test_base().await?;
        let owner = get_owner(&stack).await;
        let path = RepositoryHelper::repo_dir(SPACE, REPO_NAME);

        let refs_hash = &last_commit_oid(path.clone())?;
        info!("{:?}", refs_hash);

        let commit = git_read_commit_object(path.clone(), refs_hash)?;
        info!("commit {:?}", commit);
        let tree_id = commit.tree;
        let tree_result = git_exec_base(path.clone(), ["cat-file", "-p", &tree_id])?;
        info!("commit {:?}", tree_result);
        let tree_array: Vec<GitLsTreeResultTreeData> = tree_result
            .lines()
            .map(|line| git_ls_object_parse(line).unwrap())
            .collect();
        let mut tree: Vec<TreeItem> = Vec::new();
        for item in tree_array {
            tree.push(TreeItem {
                mode: item.file_mode,
                hash: item.file_hash,
                file_name: item.file_name,
                file_type: item.git_object_type,
            })
        }
        info!("tree {:?}", tree);
        let tree_object = Tree::create(owner, tree_id, tree);
        let object_id = tree_object.desc().calculate_id();
        let _ = put_object(&stack, &tree_object).await;

        let buf = get_object(&stack, object_id).await?;
        let tree_object2 = Tree::clone_from_slice(&buf)?;

        info!("tree_id  {:?}", tree_object2.tree_id().to_string());
        assert_eq!(
            tree_object2.tree_id().to_string(),
            tree_object.tree_id().to_string()
        );
        info!("tree_object2  {:?}", tree_object2.tree());
        Ok(())
    }

    #[async_std::test]
    async fn repository_loop_tree_object_insert() -> BuckyResult<()> {
        let stack = test_base().await?;
        let owner = get_owner(&stack).await;
        let path = RepositoryHelper::repo_dir(SPACE, REPO_NAME);
        let refs_hash = &last_commit_oid(path.clone())?;
        info!("{:?}", refs_hash);
        let commit = git_read_commit_object(path.clone(), refs_hash)?;
        info!("commit {:?}", commit);
        let root_tree_id = commit.tree;

        let _r = root_tree_object_loop_insert_map(
            &stack,
            owner,
            path.clone(),
            &root_tree_id,
            SPACE,
            REPO_NAME,
        )
        .await;
        Ok(())
    }

    // 写入所有的tree对象
    #[async_std::test]
    async fn test_commits_root_tree_object_loop_insert_map() -> BuckyResult<()> {
        let stack = test_base().await?;
        let owner = get_owner(&stack).await;
        let path = RepositoryHelper::repo_dir(SPACE, REPO_NAME);

        let commit_oid = &last_commit_oid(path.clone())?;
        commits_root_tree_object_loop_insert_map(
            &stack,
            owner,
            path.clone(),
            commit_oid,
            SPACE,
            REPO_NAME,
        )
        .await?;
        info!("commits_root_tree_object_loop_insert_map ok");
        Ok(())
    }

    #[async_std::test]
    async fn test_read_repository_home_files_by_markup_clean() -> BuckyResult<()> {
        let branch = "master";
        let stack = test_base().await?;
        let path = RepositoryHelper::repo_dir(SPACE, REPO_NAME);
        let lastest_oid = last_commit_oid(path.clone())?;
        let sub_dir = "";
        let mut treefile = TreeFileHelper::new(
            stack,
            SPACE.to_string(),
            REPO_NAME.to_string(),
            lastest_oid.clone(),
            branch.to_string(),
            sub_dir.to_string(),
        )
        .await?;
        treefile.prepare().await?;
        let result_main_file_map = treefile.start().await?;
        // result
        info!("len of result {}", result_main_file_map.len());
        for (key, value) in result_main_file_map.clone().into_iter() {
            info!(
                "[{}] ({})  {}  {}",
                key, value.file_type, value.message, value.hash
            );
        }
        // info!("second loop run ~~~");
        // treefile._print();

        Ok(())
    }

    // 子目录（sub tree)下的处理
    #[async_std::test]
    async fn test_read_repository_home_files_subdir() -> BuckyResult<()> {
        let branch = "master";
        let stack = test_base().await?;
        let path = RepositoryHelper::repo_dir(SPACE, REPO_NAME);
        let lastest_oid = git_lastest_commit_oid(path.clone(), branch)?;
        let sub_dir = "internal/path";

        let mut treefile = TreeFileHelper::new(
            stack,
            SPACE.to_string(),
            REPO_NAME.to_string(),
            lastest_oid.clone(),
            branch.to_string(),
            sub_dir.to_string(),
        )
        .await?;
        treefile.prepare().await?;
        let result_main_file_map = treefile.start().await?;
        info!("len of result {}", result_main_file_map.len());
        for (key, value) in result_main_file_map.clone().into_iter() {
            info!("[{}]  {}  {}", key, value.message, value.hash);
        }
        Ok(())
    }
}
