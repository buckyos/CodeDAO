# code-dao-service RootState

```
|--organization
|   |--${org.name}
|   |   |--info-->organization
|   |`  |--members
|   |   |   |--${member.name}-->org.member
|   |   |--repos
|   |   |   |--${repo.name}-->repo
|--repo
|   |--${repo.name}
|   |   |--repo--> RepoObject
|   |   |--branchs
|   |   |   |--${branch_name}
|   |   |   |   |--info-->BranchInfo
|   |   |   |   |--commits
|   |   |   |   |   |--${commit.id}-->commit
|   |   |   |   |--stars
|   |   |   |   |   |--${user.id}-->RepositoryStar
|   |   |--members
|   |   |   |--${user.peopleid}-->repo.member
|   |   |--trees
|   |   |   |--${tree.id}-->Tree
|   |   |--issues
|   |   |   |--${topic.id}-->Issue // topic.id: Autoincrement
|--user
|   |--info -->UserInfo

```
