/app
    |--organization
    |   |--list // ORG_LIST_PATH
    |   |   |--${org.name}-->organization
    |   |--member // ORG_MEMBER_PATH
    |   |   |--${org.name}
    |   |   |   |--${member.name}-->org.member
    |   |--repo // ORG_REPO_PATH
    |   |   |--${author.name}
    |   |   |   |--${repo.name}-->repo
    |--repo // REPOSITORY_PATH
    |   |--${author.name}
    |   |   |--${repo.name}
    |   |   |   |--repo-->repo
    |   |   |   |--commit
    |   |   |   |   |--${commit.id}-->commit
    |   |   |   |--refs
    |   |   |   |   |--${ref.name}-->branch
    |   |   |   |--star
    |   |   |   |   |--${user.name}-->RepositoryStar
    |   |   |   |--member
    |   |   |   |   |--${user.peopleid}-->repo.member
    |   |   |   |--tree
    |   |   |   |   |--${tree.id}-->Tree
    |   |   |   |--issue
    |   |   |   |   |--${topic.id}-->Issue
    |   |   |   |--issue_comment
    |   |   |   |   |--${topic.id}
    |   |   |   |   |   |--${comment.id}-->Issue
    |   |   |   |--merge
    |   |   |   |   |--${merge.id}-->MergeRequest
    |--user
    |   |--list // USER_LIST_PATH
    |   |   |--${user.peopleid}-->UserInfo
    |--username
    |   |--list // USER_NAME_LIST_PATH
    |   |   |--${user.name}-->UserInfo
