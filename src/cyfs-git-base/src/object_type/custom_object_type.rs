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
    Blob,
    GitText = 32810, // www的ts也指定了这个number
}
