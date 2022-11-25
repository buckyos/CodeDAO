use crate::*;
use async_std::sync::Arc;
use cyfs_base::*;
use cyfs_lib::*;
use log::*;
use std::path::PathBuf;

pub struct StackUtil {
    stack: Arc<SharedCyfsStack>,
    owner: ObjectId,
    ood_id: ObjectId,
    device_id: DeviceId,
}

impl StackUtil {
    pub fn new(
        stack: Arc<SharedCyfsStack>,
        owner: ObjectId,
        ood_id: ObjectId,
        // device_id: DeviceId,
    ) -> Self {
        let device_id = stack.local_device().desc().device_id();
        Self {
            stack,
            owner,
            ood_id,
            device_id,
        }
    }

    pub async fn upload(&self, local_path: PathBuf) -> BuckyResult<()> {
        let result = self
            .stack
            .trans()
            .publish_file(&TransPublishFileOutputRequest {
                common: NDNOutputRequestCommon {
                    dec_id: Some(dec_id()),
                    level: NDNAPILevel::Router,
                    referer_object: vec![],
                    flags: 0,
                    target: None,
                    req_path: None,
                },
                owner: self.owner,
                local_path,
                chunk_size: 1024 * 1024 * 4,
                file_id: None,
                dirs: None,
            })
            .await?;
        let file_id = result.file_id;

        let object_raw = get_object(&self.stack, file_id).await?;
        let access = AccessString::full();
        let _r = self
            .stack
            .non_service()
            .put_object(NONPutObjectOutputRequest {
                common: NONOutputRequestCommon {
                    req_path: None,
                    source: None,
                    dec_id: None,
                    level: NONAPILevel::Router,
                    target: Some(self.ood_id),
                    flags: 0,
                },
                object: NONObjectInfo {
                    object_id: file_id.clone(),
                    object_raw,
                    object: None,
                },
                access: Some(access),
            })
            .await;
        let task = self
            .stack
            .trans()
            .create_task(&TransCreateTaskOutputRequest {
                common: NDNOutputRequestCommon {
                    req_path: None,
                    dec_id: Some(dec_id()),
                    level: NDNAPILevel::NDC,
                    target: Some(self.ood_id),
                    referer_object: vec![],
                    flags: 0,
                },
                object_id: file_id,
                // 保存到的本地目录or文件
                local_path: PathBuf::new(),
                device_list: vec![self.device_id.clone()],
                context_id: None,
                // 任务创建完成之后自动启动任务
                auto_start: true,
            })
            .await?;
        let task_id = task.task_id;

        loop {
            let state = self
                .stack
                .trans()
                .get_task_state(&TransGetTaskStateOutputRequest {
                    common: NDNOutputRequestCommon {
                        req_path: None,
                        dec_id: Some(dec_id()),
                        level: NDNAPILevel::NDC,
                        target: Some(self.ood_id),
                        referer_object: vec![],
                        flags: 0,
                    },
                    task_id: task_id.clone(),
                })
                .await?;

            match state {
                TransTaskState::Pending => {}
                TransTaskState::Downloading(_) => {}
                TransTaskState::Paused | TransTaskState::Canceled => {
                    let msg = format!("download {} task abnormal exit.", file_id.to_string());
                    error!("{}", msg.as_str());
                    return Err(BuckyError::new(BuckyErrorCode::Failed, msg));
                }
                TransTaskState::Finished(_) => {
                    debug!("file task finish {}", file_id.to_string());
                    break;
                }
                TransTaskState::Err(err) => {
                    let msg = format!("download {} failed.{}", file_id.to_string(), err);
                    error!("{}", msg.as_str());
                    return Err(BuckyError::new(err, msg));
                }
            }
            // if matches!(status, TransTaskState::Finished(0)) {
            //     debug!("get_task_state  check status finished");
            //     break;
            // }
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        }

        self.stack
            .trans()
            .delete_task(&TransTaskOutputRequest {
                common: NDNOutputRequestCommon {
                    req_path: None,
                    dec_id: None,
                    level: NDNAPILevel::NDC,
                    target: Some(self.ood_id),
                    referer_object: vec![],
                    flags: 0,
                },
                task_id,
            })
            .await?;

        Ok(())
    }
}
