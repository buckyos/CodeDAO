
use cyfs_base::*;


#[repr(u16)]
pub enum CustomObjType {
    Test = OBJECT_TYPE_DECAPP_START + 2,
    Repository,
    UserInfo,
    Organization,
    Commit,
    Issue,
    RepositoryMember,
    RepositoryStar,
    MergeRequest,
    OrganizationMember,
    RepositoryBranch,
    Tree,
    GitText = 32810   // GitText  要指定这个number， 这个是内置的acl放行过的对象
}
