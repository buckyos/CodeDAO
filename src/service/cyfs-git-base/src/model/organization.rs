use cyfs_lib::*;
use cyfs_base::*;
use log::*;
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use crate::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationData {
    pub name: String,
    pub description: String,
    pub email: String,
    pub avatar: String,
    pub creator: String,
    pub org_id: String,

}


pub async fn insert_organization(stack: &Arc<SharedCyfsStack>, organization : &Organization) -> BuckyResult<()> {
    let _id = organization.id();
    
    let org_key  = format!("{}/{}" ,ORG_LIST_PATH, organization.name());
    let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
    let result = env.get_by_path(&org_key).await?;
    if result.is_some() {
        error!("organization[{}] was already created", org_key);
        return Err(BuckyError::new(BuckyErrorCode::AlreadyExists, format!("organization[{}] was already created", &org_key)))
    }
    
    let object_id = organization.desc().calculate_id();

    let r = env.set_with_path(&org_key, &object_id, None, true).await?;
    info!("env set_with_path repository{:?}  result:{:?}", org_key, r);

    let root = env.commit().await;
    println!("add repository commit: {:?}", root);
    Ok(())
}

pub async fn insert_organization_member(stack: &Arc<SharedCyfsStack>, organization_member: &OrganizationMember) -> BuckyResult<()>  {
    let key  = format!("{}/{}/{}" ,ORG_MEMBER_PATH, organization_member.organization_name(),organization_member.user_name());
    
    let env = stack.root_state_stub(None, Some(dec_id())).create_path_op_env().await?;
    let result = env.get_by_path(&key).await?;
    if result.is_some() {
        error!("user [{}] was already in org [{}]", organization_member.user_name(), organization_member.organization_name());
        return Err(BuckyError::new(BuckyErrorCode::AlreadyExists, format!("user [{}] was already in org [{}]", organization_member.user_name(), organization_member.organization_name())))
    }

    let object_id = organization_member.desc().calculate_id();

    let r = env.set_with_path(&key, &object_id, None, true).await?;
    info!("env set_with_path repository{:?}  result:{:?}", key, r);

    let root = env.commit().await;
    println!("add repository commit: {:?}", root);
    Ok(())
}