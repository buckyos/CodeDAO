use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_debug::*;
use cyfs_git_base::*;
use cyfs_lib::*;
use log::*;

#[async_std::main]
async fn main() -> BuckyResult<()> {
    CyfsLoggerBuilder::new_app("cyfs-git-service-manager")
        .level("info")
        .console("info")
        .enable_bdt(Some("error"), Some("error"))
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

    delete_service_all_repository().await;
    delete_all_repository().await;

    Ok(())
}
async fn delete_all_repository() -> BuckyResult<()> {
    let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
    stack.wait_online(None).await.unwrap();
    let env = stack
        .root_state_stub(None, Some(dec_id()))
        .create_path_op_env()
        .await?;
    env.remove_with_path(REPOSITORY_PATH, None).await?;
    let r = env.commit().await?;

    Ok(())
}
async fn delete_service_all_repository() -> BuckyResult<()> {
    let stack = Arc::new(
        SharedCyfsStack::open_default(Some(service_dec_id()))
            .await
            .unwrap(),
    );
    stack.wait_online(None).await.unwrap();
    let env = stack
        .root_state_stub(None, Some(service_dec_id()))
        .create_path_op_env()
        .await?;
    env.remove_with_path(REPOSITORY_PATH, None).await?;
    let r = env.commit().await?;

    Ok(())
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[async_std::test]
    async fn test_delete_service_user() -> BuckyResult<()> {
        CyfsLoggerBuilder::new_app("cyfs-git-service-manager")
            .level("info")
            .console("info")
            .enable_bdt(Some("error"), Some("error"))
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

        let stack = Arc::new(
            SharedCyfsStack::open_default(Some(service_dec_id()))
                .await
                .unwrap(),
        );
        stack.wait_online(None).await.unwrap();

        let env = stack
            .root_state_stub(None, Some(service_dec_id()))
            .create_path_op_env()
            .await?;
        env.remove_with_path(USER_LIST_PATH, None).await?;
        let r = env.commit().await?;

        Ok(())
    }

    #[async_std::test]
    async fn test_delete_dec_app_user() -> BuckyResult<()> {
        CyfsLoggerBuilder::new_app("cyfs-git-service-manager")
            .level("info")
            .console("info")
            .enable_bdt(Some("error"), Some("error"))
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

        let stack = Arc::new(SharedCyfsStack::open_default(Some(dec_id())).await.unwrap());
        stack.wait_online(None).await.unwrap();

        let env = stack
            .root_state_stub(None, Some(dec_id()))
            .create_path_op_env()
            .await?;
        env.remove_with_path(USER_LIST_PATH, None).await?;
        let r = env.commit().await?;

        Ok(())
    }
}
