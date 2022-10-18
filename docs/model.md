
# object map path 业务路径

user_id == owner == people


## 仓库对象
/app/repo/<space>/<name>/repo  -> Repository 

## commit 
/app/repo/<space>/<name>/commit/<oid>

## issue 
/app/repo/<space>/<name>/issue/<topic_id>/<comment_id>

topic_id 是自增数字  
comment_id 是object_id
<!-- 
/app/repo/<space>/<name>/issue/<id>/0  -> Topic(详情)
/app/repo/<space>/<name>/issue/<id>/<comment_id> -->

## member 
/app/repo/<space>/<name>/member/<user_id>

## star 
/app/repo/<space>/<name>/star/<user_id>


## pull request 
/app/repo/<space>/<name>/merge/<merge_id>


## 用户 init name
/app/user/userlist/<owner_id>
<!-- /app/user_name/<name> -->

## group
/app/organization/list/<group_name>
/app/organization/member/<group_name>/<user_id>
/app/organization/repo/<group_name>/<repo_name>



# test info


### bbb owner:
5r4MYfFJayyaoMHJiZjj6BtPMsNPYaDwADwXyJSAPSHX