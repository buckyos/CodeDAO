/// post请求 text object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitTextDescContent {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitTextBodyContent {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub header: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoDescContent {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryDescContent {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub init: i32,
    #[prost(int32, tag="4")]
    pub is_private: i32,
    #[prost(string, tag="5")]
    pub fork_from_id: ::prost::alloc::string::String,
    /// 拥有者可以是用户或组
    #[prost(string, tag="6")]
    pub author_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub author_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitDescContent {
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub tree_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub payload: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub author: ::core::option::Option<CommitSignature>,
    ///string parent2 = 7;
    #[prost(message, optional, tag="6")]
    pub committer: ::core::option::Option<CommitSignature>,
}
/// match git2 stcurt
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSignature {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub when: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationDescContent {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// repeated bytes members = 2;
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub avatar: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationMemberDescContent {
    #[prost(string, tag="1")]
    pub organization_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueDescContent {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub issue_type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user_name: ::prost::alloc::string::String,
    /// 在仓库中的序号， 从1开始
    #[prost(string, tag="6")]
    pub id_in_repo: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryMemberDescContent {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub repository_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub repository_author_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub repository_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryStarDescContent {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub repository_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub repository_author_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub repository_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeRequestDescContent {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub origin_branch: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub target_branch: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub merge_type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub repository_author_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub repository_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryBranchDescContent {
    #[prost(string, tag="1")]
    pub repository_author_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub repository_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ref_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ref_hash: ::prost::alloc::string::String,
}
/// repository tree object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeDescContent {
    #[prost(string, tag="1")]
    pub tree_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub tree: ::prost::alloc::vec::Vec<TreeItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeItem {
    #[prost(string, tag="1")]
    pub mode: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub file_type: ::prost::alloc::string::String,
}
