# code-dao-service RootState

```
|--organization
|   |--${org.name}
|   |   |--info --> Organization
|   |`  |--members
|   |   |   |--${people_id} --> OrgMember
|   |   |--members_consensus
|   |   |   |--raw --> MemberList
|   |   |   |--members --> ConsensusMemberList
|   |   |--repos
|   |   |   |--${repo.name} --> Repo
|   |   |--repos_consensus
|   |   |   |--raw --> RepoList
|   |   |   |--members --> ConsensusMemberList
|--repo
|   |--${repo.name}
|   |   |--info --> Repo
|   |   |--branches
|   |   |   |--${branch_name}
|   |   |   |   |--info --> BranchInfo
|   |   |   |   |--commits_consensus
|   |   |   |   |   |--raw --> CommitList
|   |   |   |   |   |--members --> ConsensusMemberList
|   |   |   |   |--commits
|   |   |   |   |   |--${commit.id} --> Commit
|   |   |   |   |--stars
|   |   |   |   |   |--info --> StartsInfo
|   |   |   |   |   |--list
|   |   |   |   |   |   |--${people_id} --> RepositoryStar
|   |   |--branches_consensus
|   |   |   |--raw --> BranchList
|   |   |   |--members --> ConsensusMemberList
|   |   |--members
|   |   |   |--${people_id} --> RepoMember
|   |   |--members_consensus
|   |   |   |--raw -->MemberList
|   |   |   |--members --> ConsensusMemberList
|   |   |--trees // git trees
|   |   |   |--${tree_id} --> Tree
|   |   |--issues
|   |   |   |--${topic_id} --> Issue // topic_id: Autoincrement
|   |   |--issues_consensus
|   |   |   |--raw --> IssueList
|   |   |   |--members --> ConsensusMemberList
|--user
|   |--${people_id} --> UserInfo
```

# square-service RootState

```
|--organization
|   |--${org.name}
|   |   |--info --> Organization
|--repo
|   |--${repo.name}
|   |   |--info --> Repo
|--user
|   |--list
|   |   |--${people_id} --> UserInfo
|   |--alia_list
|   |   |--${username} --> UserInfo
```
