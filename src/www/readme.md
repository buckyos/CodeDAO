# git-server

去中心化 git

## 开始

```sh
git clone 待补充
```

## 安装依赖

```sh
cd git-server
yarn
```

## 本地开发

```sh
yarn www:dev
```

## lint 检查

```sh
yarn lint
```

## lint 自动修复

```sh
yarn lint-覅x
```

## 构建生产

## 目录结构

.  
├── 3rd   
    └── protoc //对象protobuf处理
├── doc     // 文档
├── tools   // 工具脚本, 用来拷贝 types 和 cyfs 文件  
├── package.json // cyfs 打包上传依赖的 node 库  
├── readme.md  
└── www // 前端代码
    └── src //
        ├── apis // 接口文件
        ├── assets // 静态资源文件
        ├── components // 组件文件
        ├── constants // 常量文件
        ├── i18n // 国际化语言包
        ├── stores // 全局状态文件
        ├── pages // 页面文件
        ├── routers // 路由文件
        ├── styles // 样式文件
        ├── types // 类型文件
        └── utils // 工具文件

## ACL 权限

如果安装时没有，就修改一下 acl.toml

```
* acl 配置
** 开发测试
[cyfs-git]
post-object = {action="*-post-object",res="/dec_app/9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r/32810",access="accept"}
put-repository = {action="*-put-object",res="/dec_app/9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r/33498",access="accept"}
git-get-object = {action = "*-get", res = "/9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r/**", access = "accept"}
put-object = {action = "*-put-object", res = "/dec_app/9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r/**", access = "handler"}


** nightly环境
[cyfs-git]
post-object = {action="*-post-object",res="/dec_app/9tGpLNnYywrCAWoCcyhAcLZtrQpDZtRAg3ai2w47aap2/32810",access="accept"}
put-repository = {action="*-put-object",res="/dec_app/9tGpLNnYywrCAWoCcyhAcLZtrQpDZtRAg3ai2w47aap2/33498",access="accept"}
git-get-object = {action = "*-get", res = "/9tGpLNnYywrCAWoCcyhAcLZtrQpDZtRAg3ai2w47aap2/**", access = "accept"}
put-object = {action = "*-put-object", res = "/dec_app/9tGpLNnYywrCAWoCcyhAcLZtrQpDZtRAg3ai2w47aap2/**", access = "handler"}

```
