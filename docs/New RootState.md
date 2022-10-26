# code-dao-service RootState

```
|--consensus
|   |--... // 共识框架占用
|--app
    |--organization
    |   |--${org.id}
    |   |   |--members // cache
    |   |   |   |--${member.people_id} --> OrgMember
    |   |   |--members_consensus
    |   |   |   |--roles
    |   |   |   |   |--${member.people_id} --> MemberRole
    |   |   |--info
    |   |   |   |--raw --> Organization
    |   |   |   |--.members --> members_consensus
    |   |   |--repos_consensus
    |   |   |   |--raw --> Set<repo.id>
    |   |   |   |--.members --> members_consensus
    |--repo
    |   |--${repo.id}
    |   |   |--members // cache
    |   |   |   |--${member.people_id} --> RepoMember
    |   |   |--members_consensus
    |   |   |   |--roles
    |   |   |   |   |--${member.people_id} --> MemberRole
    |   |   |--info
    |   |   |   |--raw --> Repo
    |   |   |   |--.members --> members_consensus
    |   |   |--branches
    |   |   |   |--${branch_name}
    |   |   |   |   |--info
    |   |   |   |   |   |--raw --> BranchInfo
    |   |   |   |   |   |--.members --> members_consensus
    |   |   |   |   |--changes
    |   |   |   |   |   |--.members --> members_consensus
    |   |   |   |   |   |--process_status --> // WillCommit(id), Commiting(id), Standby
    |   |   |   |   |   |--commits --> Set<Commit.id>
    |   |   |   |   |   |--merges --> Set<Merge.id>
    |   |   |--branches_consensus
    |   |   |   |--raw --> BranchList
    |   |   |   |--members --> members_consensus
    |   |   |--stars
    |   |   |   |--${user.people_id}
    |   |   |   |   |--raw --> RepositoryStar
    |   |   |   |   |--.members --> members_consensus
    |   |   |--stars_consensus
    |   |   |   |--raw --> Set<user.people_id>
    |   |   |   |--.members --> members_consensus
    |   |   |--trees // ??
    |   |   |   |--${tree_id} --> Tree
    |   |   |--issues
    |   |   |   |--${topic_id} // topic_id: Autoincrement
    |   |   |   |   |--info
    |   |   |   |   |   |--raw --> Issue
    |   |   |   |   |   |--.members --> members_consensus
    |   |   |   |   |--comments
    |   |   |   |   |   |--${comment_id} // Autoincrement
    |   |   |   |   |   |   |--info
    |   |   |   |   |   |   |   |--raw --> IssueComment
    |   |   |   |   |   |   |   |--.members --> members_consensus
    |   |   |   |   |--comments_consensus
    |   |   |   |   |   |--raw --> number // comments.count
    |   |   |   |   |   |--.members --> members_consensus
    |   |   |--issues_consensus
    |   |   |   |--raw --> number // issues.count
    |   |   |   |--.members --> members_consensus
```

# square-service RootState

```
Coming soon
```
