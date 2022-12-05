use crate::post::post_object;
use crate::push::Push;
use async_std::sync::Arc;
use clap::*;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;

pub struct Service {
    stack: Arc<SharedCyfsStack>,
    push: Push,
}

impl Service {
    pub async fn new(stack: Arc<SharedCyfsStack>) -> Self {
        let name = "2022_1110";
        let test_dir_path = format!("/home/aa/test/{}", name);
        let ood = get_ood_device(&stack).await;
        let owner = owner(&stack);
        // init stack util helper
        // TODO  move put object to this
        let stack_util = Arc::new(StackUtil::new(
            Arc::clone(&stack),
            owner.clone(),
            ood.clone(),
        ));
        let name = format!("{}/{}", owner.to_string(), name);

        let branch = "main".to_string();
        let repo = git2::Repository::open(test_dir_path).expect("open repo failed");
        let push = Push::new(
            Arc::new(repo),
            Arc::clone(&stack),
            stack_util,
            name,
            branch,
            ood,
            owner,
        );

        Self { stack, push }
    }

    // create repository
    pub async fn create(&self, name: &str) {
        let data = serde_json::json!({
            "name": name,
            "description":"aaa",
            "is_private":0,
            "author_type":"user",
        });
        let data = data.to_string();
        post_object(&self.stack, "repo/init", &data).await;
    }

    // push
    pub async fn push(&self) -> CodedaoResult<()> {
        self.push.push().await
    }

    pub async fn debug(&self) -> CodedaoResult<()> {
        self.push.debug().await
    }
}
