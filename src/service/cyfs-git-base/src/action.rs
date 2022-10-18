

use cyfs_lib::*;
use cyfs_base::*;
use async_std::sync::Arc;
use std::{path::PathBuf, str::FromStr};
use log::*;
use crate::*;


pub async fn put_object<DC, BC>(stack: &Arc<SharedCyfsStack>, object: &NamedObjectBase<NamedObjType<DC, BC>>) -> BuckyResult<NONPutObjectOutputResponse>
where
    DC: RawEncode + DescContent + Sync + Send + Clone,
    BC: Sync + Send + Clone + RawEncode + BodyContent,
{
    put_object_target(stack, object, None, None).await
}

pub async fn put_object_target<DC, BC>(
    stack: &Arc<SharedCyfsStack>, 
    object: &NamedObjectBase<NamedObjType<DC, BC>>, 
    target_device: Option<ObjectId>,
    req_path: Option<String>,
) -> BuckyResult<NONPutObjectOutputResponse>
where
    DC: RawEncode + DescContent + Sync + Send + Clone,
    BC: Sync + Send + Clone + RawEncode + BodyContent,
{
    let r = stack.non_service().put_object(NONPutObjectOutputRequest {
        common: NONOutputRequestCommon {
            req_path,
	    source: None,
            dec_id: None,
            level: NONAPILevel::Router,
            target: target_device,
            flags: 0
        },
        object: NONObjectInfo {
            object_id:  object.desc().object_id().clone(),
            object_raw: object.to_vec().unwrap(),
            object: None
        },
	access: None,
    }).await;
    r
}



pub async fn get_object(stack: &Arc<SharedCyfsStack>, object_id: ObjectId) -> BuckyResult<Vec<u8>> {
    let r = get_object_target(stack, object_id, None).await?;
    Ok(r)
}

pub async fn get_object_target(stack: &Arc<SharedCyfsStack>, object_id: ObjectId, target: Option<ObjectId>) -> BuckyResult<Vec<u8>> {
    let r = stack.non_service().get_object(NONGetObjectOutputRequest {
        common: NONOutputRequestCommon {
            req_path: None,
            source: None,
            dec_id: None,
            level: NONAPILevel::Router,
            target,
            flags: 0
        },
        object_id,
        inner_path: None,
    }).await?;
    let buf = r.object.object_raw;
    
    Ok(buf)
}

pub async fn delete_object(stack: &Arc<SharedCyfsStack>, object_id: ObjectId) -> BuckyResult<()> {
    let _r = stack.non_service().delete_object(NONDeleteObjectOutputRequest {
        common: NONOutputRequestCommon {
            req_path: None,
            source: None,
            dec_id: None,
            level: NONAPILevel::Router,
            target: None,
            flags: 0
        },
        object_id,
        inner_path: None,
    }).await?;
    
    Ok(())
}

pub async fn get_owner(stack: &Arc<SharedCyfsStack>) -> ObjectId {
    let device = stack.local_device();
    let owner = device
        .desc()
        .owner()
        .to_owned()
        .unwrap_or_else(|| device.desc().calculate_id());
    owner
}

pub fn owner(stack: &Arc<SharedCyfsStack>) -> ObjectId {
    let device = stack.local_device();
    let owner = device
        .desc()
        .owner()
        .to_owned()
        .unwrap_or_else(|| device.desc().calculate_id());
    owner
}

pub fn get_local_device(stack: &Arc<SharedCyfsStack>) -> ObjectId {
    let device = stack.local_device().desc().device_id();
    let device = device.object_id();
    *device
    // let owner = device
    //     .desc()
    //     .owner()
    //     .to_owned()
    //     .unwrap_or_else(|| device.desc().calculate_id());
    // owner
}

pub async fn get_ood_device(stack: &Arc<SharedCyfsStack>) -> ObjectId {
    let req = UtilGetDeviceStaticInfoRequest::new();
    let result = stack.util_service().get_device_static_info(req).await.unwrap();
    let ood = result.info.ood_device_id;
    ObjectId::try_from(ood).unwrap()
}

pub async fn get_target_device(stack: &Arc<SharedCyfsStack>, owner_id: ObjectId) -> BuckyResult<ObjectId> {
    let result = stack.util()
    .resolve_ood(UtilResolveOODRequest{
        common: UtilOutputRequestCommon{
            req_path: None,
            dec_id: Some(dec_id()),
            target: None,
            flags: 0,
        },
        object_id: owner_id,
        owner_id: Some(owner_id),
    }).await?;
    let device = &result.device_list[0];
    let device = device.object_id();
    Ok(*device)
}

pub async fn create_path_op_env(stack: &Arc<SharedCyfsStack>) -> BuckyResult<PathOpEnvStub>{
    let ood = get_ood_device(stack).await;
    let env = stack.root_state_stub(Some(ood), Some(dec_id())).create_path_op_env().await?;
    Ok(env)
}




pub async fn post_target(stack: &Arc<SharedCyfsStack>, target:ObjectId, route:&str, data: &str, req_path: Option<String>) -> BuckyResult<(String, String)> {
    let text = GitText::create(
        // TODO  owner global
        Some(get_owner(stack).await),
        route.to_string(), 
        "header".to_string(), 
        data.to_string());

    let r = stack.non_service().post_object(NONPostObjectOutputRequest{
        common: NONOutputRequestCommon {
            req_path,
            source: None,
            dec_id: Some(dec_id()),
            level: NONAPILevel::Router,
            target: Some(target),
            flags: 0
        },
        object: NONObjectInfo{
            object_id:  text.desc().object_id().clone(),
            object_raw: text.to_vec().unwrap(),
            object: None,

        }
    }).await?;
    let buf = r.object.unwrap().object_raw;
    let response_text = GitText::clone_from_slice(&buf).unwrap() as GitText;
    let id = response_text.id().to_string();
    let value = response_text.value().to_string();
    Ok((id, value))
}

// post_special_object_service
// 非 text 类型object 传输到service应用
// 主要是为了在service 上做put处理
pub async fn post_special_object_service<DC, BC>(
    stack: &Arc<SharedCyfsStack>, 
    object: &NamedObjectBase<NamedObjType<DC, BC>>, 
    special_name: impl Into<String>,
) -> BuckyResult<()>
where
    DC: RawEncode + DescContent + Sync + Send + Clone,
    BC: Sync + Send + Clone + RawEncode + BodyContent,
{
    let req_path = Some(format!("{}/{}/{}", DEC_SERVICE_HANDLER, "object", special_name.into()));
    let req_path = Some(RequestGlobalStatePath::new(Some(service_dec_id()), req_path).format_string());
    let target = Some(service_ood_device());


    let r = stack.non_service().post_object(NONPostObjectOutputRequest{
        common: NONOutputRequestCommon {
            req_path,
            source: None,
            dec_id: Some(dec_id()),
            level: NONAPILevel::Router,
            target,
            flags: 0
        },
        object: NONObjectInfo{
            object_id:  object.desc().object_id().clone(),
            object_raw: object.to_vec().unwrap(),
            object: None,
        }
    }).await?;

    // 解析返回的json
    if let Some(ref object) = r.object {
        let buf = &object.object_raw;
        let text = GitText::clone_from_slice(buf)?;
        let value = text.value();
        info!("resp value {} ", value);
        let value = serde_json::Value::from_str(value).unwrap();
        let status = value["status"].as_i64();
        info!("resp status {:?} ", status);
        let status = status.unwrap();

        if status != 0  {
            if let Some(msg) = value["msg"].as_str() {
                error!("service special object failed: {}", msg);
                return Err(BuckyError::new(BuckyErrorCode::InternalError, msg))
            }
        }
        Ok(())
    } else {
        Err(BuckyError::new(BuckyErrorCode::Failed, "empty response object"))
    }
}


// post_special_object_other_ood
// 非 text 类型object 传输到
// other ood 上做put处理
pub async fn post_special_object_target_ood<DC, BC>(
    stack: &Arc<SharedCyfsStack>, 
    object: &NamedObjectBase<NamedObjType<DC, BC>>,
    target: Option<ObjectId>,
    special_name: impl Into<String>,
) -> BuckyResult<()>
where
    DC: RawEncode + DescContent + Sync + Send + Clone,
    BC: Sync + Send + Clone + RawEncode + BodyContent,
{
    let req_path = Some(format!("{}/{}/{}", DEC_APP_HANDLER, "object", special_name.into()));
    let req_path = Some(RequestGlobalStatePath::new(Some(dec_id()), req_path).format_string());
    info!("Send sepcial object request ==> target OOD, req_path {:?}", req_path);
    let r = stack.non_service().post_object(NONPostObjectOutputRequest{
        common: NONOutputRequestCommon {
            req_path,
            source: None,
            dec_id: Some(dec_id()),
            level: NONAPILevel::Router,
            target,
            flags: 0
        },
        object: NONObjectInfo{
            object_id:  object.desc().object_id().clone(),
            object_raw: object.to_vec().unwrap(),
            object: None,
        }
    }).await?;

    // 解析返回的json
    if let Some(ref object) = r.object {
        let buf = &object.object_raw;
        let text = GitText::clone_from_slice(buf)?;
        let value = text.value();
        info!("resp value {} ", value);
        let value = serde_json::Value::from_str(value).unwrap();
        let status = value["status"].as_i64();
        info!("resp status {:?} ", status);
        let status = status.unwrap();

        if status != 0  {
            if let Some(msg) = value["msg"].as_str() {
                error!("service special object failed: {}", msg);
                return Err(BuckyError::new(BuckyErrorCode::InternalError, msg))
            }
        }
        Ok(())
    } else {
        Err(BuckyError::new(BuckyErrorCode::Failed, "empty response object"))
    }
}


use once_cell::sync::OnceCell;
pub struct StackActionStruct {
    pub stack: Arc<SharedCyfsStack>,
    pub owner: ObjectId,
    pub dec_id: ObjectId,
}
impl StackActionStruct {
    // 向广场ood， 发post 
    pub async fn post_service(&self, route:&str, data: &str) -> BuckyResult<(String, String)> {
        let req_path = Some(format!("{}/api", DEC_SERVICE_HANDLER));
        let req_path = Some(RequestGlobalStatePath::new(Some(service_dec_id()), req_path).format_string());
	//info!("Send a request ==> service, req_path {:?}", req_path);
        let target = Some(service_ood_device());

        let text = GitText::create(
            // TODO  owner global
            Some(get_owner(&self.stack).await),
            route.to_string(), 
            "header".to_string(), 
            data.to_string());
    
        let r = self.stack.non_service().post_object(NONPostObjectOutputRequest{
            common: NONOutputRequestCommon {
                req_path,
                source: None,
                dec_id: Some(dec_id()),
                level: NONAPILevel::Router,
                target,
                flags: 0
            },
            object: NONObjectInfo{
                object_id:  text.desc().object_id().clone(),
                object_raw: text.to_vec().unwrap(),
                object: None,
    
            }
        }).await?;
        let buf = r.object.unwrap().object_raw;
        let response_text = GitText::clone_from_slice(&buf).unwrap() as GitText;
        let id = response_text.id().to_string();
        let value = response_text.value().to_string();
        Ok((id, value))
    }
}

pub static STACK_ACTION: OnceCell<StackActionStruct> = OnceCell::new();







/// file_task
/// download file 
pub async fn file_task(stack: &Arc<SharedCyfsStack>, task_file_id: ObjectId, local_path: PathBuf, from_device_id: DeviceId, file_object_target: Option<ObjectId> ) -> BuckyResult<()>{
    // get file object first
    let _ = get_object_target(stack, task_file_id, file_object_target).await?;
    
    let result = stack.trans().create_task(&TransCreateTaskOutputRequest{
        common: NDNOutputRequestCommon{
            req_path: None,
            dec_id: Some(dec_id()),
            level: NDNAPILevel::Router,
            target: None,
            referer_object: vec![],
            flags: 0,
        },
        object_id: task_file_id,
        // 保存到的本地目录or文件
        local_path,
        device_list:vec![from_device_id],
        context_id: None,
        // 任务创建完成之后自动启动任务
        auto_start: true,
    }).await?;

    // to check the task
    let task_id = result.task_id;
    loop {
        let status = stack.trans().get_task_state(&TransGetTaskStateOutputRequest{
            common: NDNOutputRequestCommon{
                req_path: None,
                dec_id: Some(dec_id()),
                level: NDNAPILevel::Router,
                target: None,
                referer_object: vec![],
                flags: 0,
            },
            task_id: task_id.clone(),
        }).await?;
        if matches!(status, TransTaskState::Finished(0)) {
            debug!("get_task_state  check status finished");
            break;
        }
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}


pub async fn publish_file(stack: &Arc<SharedCyfsStack>, local_path: PathBuf) -> BuckyResult<ObjectId> {
    // create bundle file index(objectid)
    let response = stack.trans().publish_file(&TransPublishFileOutputRequest{
            common: NDNOutputRequestCommon{
            dec_id: Some(dec_id()),
            level: NDNAPILevel::Router,
            referer_object: vec![],
            flags: 0,
            target: None,
            req_path: None,
        },
        owner: get_owner(stack).await,
        local_path,
        chunk_size: 1024 * 1024 * 4,
        file_id: None,
        dirs: None,
    }).await?;


    let object_id = response.file_id;
    let object_raw = get_object(stack, object_id).await?;
    let ood_device = get_ood_device(stack).await;
    let access = AccessString::full();
    let _r = stack.non_service().put_object(NONPutObjectOutputRequest {
        common: NONOutputRequestCommon {
            req_path: None,
	    source: None,
            dec_id: None,
            level: NONAPILevel::Router,
            target: Some(ood_device),
            flags: 0
        },
        object: NONObjectInfo {
            object_id,
            object_raw,
            object: None
        },
	access: Some(access),
    }).await;

    Ok(response.file_id)
}
