// RootState object map path
pub const REPOSITORY_PATH: &'static str = "/app/repo/";
pub const ORG_LIST_PATH: &'static str = "/app/organization/list";
pub const ORG_MEMBER_PATH: &'static str = "/app/organization/member";
pub const ORG_REPO_PATH: &'static str = "/app/organization/repo";

pub const USER_LIST_PATH: &'static str = "/app/user/list/"; // <r>/list/<owner_id>
pub const USER_NAME_LIST_PATH: &'static str = "/app/username/list/"; // <r>/list/<name>

/// Branch
pub fn rootstate_repo_branch(repo: &str, branch: &str) -> String {
    format!("{}{}/branch/{}", REPOSITORY_PATH, repo, branch)
}

pub fn rootstate_repo_refs(author_name: &str, repo_name: &str, ref_name: &str) -> String {
    format!(
        "{}{}/{}/refs/{}",
        REPOSITORY_PATH, author_name, repo_name, ref_name
    )
}

pub fn rootstate_repo_refs_list(author_name: &str, repo_name: &str) -> String {
    format!("{}{}/{}/refs", REPOSITORY_PATH, author_name, repo_name)
}

/// commit
/// name <full_name>
/// path /app/<space>/<name>/commit/<oid>
pub fn rootstate_repo_commit(name: &str, oid: &str) -> String {
    format!("{}{}/commit/{}", REPOSITORY_PATH, name, oid)
}

/// tree
pub fn rootstate_repo_tree(author_name: &str, repo_name: &str, tree_id: &str) -> String {
    format!(
        "{}{}/{}/tree/{}",
        REPOSITORY_PATH, author_name, repo_name, tree_id
    )
}
pub fn rootstate_repo_tree2(name: &str, tree_id: &str) -> String {
    format!("{}{}/tree/{}", REPOSITORY_PATH, name, tree_id)
}
