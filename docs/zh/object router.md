# cyber-git NamedObject建模与关键路由逻辑
正式的文档需要把下面关键NamedObject的字段列出来

## RepoObject
### 创建
repo's owner cyfs runtime => repo's OOD

在 repo_manger 页面中通过ts 构造,然后发送到repo's OOD 
重要行为，创建时会请求People签名，OOD接收后会增加OOD的签名，确认已经在OOD上完成了on repo create的所有处理。

进阶（需要验证）：
repo的Owner是一个group.这时是由admin权限的用户完成创建后，投递到group的OOD
### 权限控制
只读公开（非成员只有读权限）
朋友公开
私有可申请
私有不可申请（邀请制）

### 分享
最简单：将repo object (cyfs://xxx) 通过社交软件分享给朋友
中心化聚合：
repo's OOD => cyber-git's OOD
返回cyber-git ood的签名，表示已经被接收，可以被搜索到
cyber-git的DEC Service会处理OnPut,已加入可搜索列表（也可从搜索列表中去掉）
合约聚合：
开发智能合约，实现公开列表（发布到合约中需要消耗手续费）
共识列表：
构建一条侧链，通过共识算法保证repo可被搜索到。可有更低的手续费以及更好的性能。

### 使用
得到了RepoObject后，客户端就可以基于RepoObject来获得一系列对象并进行展示
1. Current Version DirObject，目前的Repo目录结构。注意对Sub module的支持（另一个repo object或另一个git目录？)
2. Brachs (其实就是CommitObject) & History （CommitObject向前引用），History看到的Commit应该是标准的git commit,点进去后才看到包裹他的Commit Object
3. Issue List.返回状态为为Normal的Issue.Owner可以看到为Pending状态的Issue
4. MergeRequest List。
5. 简单的统计面板

### 规划
1. 对 CodeReview工具的支持
2. 与周报 / 考核系统对接
3. 与CI 对接

## CommitObject
最多的对象，Owner是作者，
对git commit的包装，使其成为一个NFT。
CommitObject和git commit是可以做单向确定转换。
基本流程：
dev's cyfs-runtime => dev's OOD => repos's OOD (=> cyber-git's OOD)

### 创建
开发者在本地使用传统git工具结合git-proxy-server构造。调用git push后构造。构造好的commit只存在当前设备即可，不用发送到ood。
### push
如果dev的ood也是repo的ood,那最简单，直接push commit到ood就好。repo's ood 收到commit后，会在on put中触发标准git逻辑,并返回成功。（注意处理失败逻辑）
否则就要先push到自己的ood上（自己的ood上一定有一个fork的repo?），然后再由自己的ood push到repo's ood,等待返回。很多情况下不会立刻push 成功，可能会触发后续的 合并请求操作。
repo's ood根据repo 配置，会把commit对象转发到cyber-git's OOD,来获取更多的排名（搜索）价值。

## MergeRequestObject
开发者在对权限有正确理解的情况下，不会往没有写权限的repo PUT CommitObject,而是准备一个MergeRequestObject来主动的发起合并请求

## IssueObject
IssueObject是CYFS里一种常见的对象（和`文章对象`很相似），这里也许有机会使用CoreObject
IssueObject的Owner是创建者，并引用RepoObject / CommitObject 等。针对已有IssueObject的跟进讨论，也是一组文章对象


IssueObject创建后投递给Repo，Repo OOD的默认逻辑可以是接收该Issue并加入到Repo的Issue List.也可以设置成需要管理员批准才会出现在IssueList
### 标准 文章对象定义：秋实在朋友圈Demo里做过
Owner,作者，时间，标题,prev(上个版本，文章可以有修改记录)
主关联对象：该文章因何而起，比如IssueObject的主关联是RepoObject,IssueObject里的回复的主关联是IssueObject，回复也可以有回复（
内容（一段富文本，可以通过cyfs:// link 别的对象）
关联对象：列出一组关联对象，不同的场景（App）在解析时，可以根据这些关联对象进行更友好的UI展示。也可以进行反向处理。

## 处理通知
可以基于超送的MSG来进行通知
超送的MSG，在用于应用通知时，应有很好的分类能力

## 支持协作开发
参考Slack的Channel模型，可以以一个对象（或一组对象）为主题，建立一个Channel，然后进行实时沟通。


