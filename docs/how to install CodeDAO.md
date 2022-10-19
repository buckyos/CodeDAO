

# 1 How to install CodeDao

## 1.1 Before install app, make sure the OOD' sets already

Make sure you install OOD.    
`ps: If your OOD origin system is windows or Nas, you need to install git binary first(version >= 2.3).`

## 1.2 Install DEC App in CYFS browser or CyberChat
CYFS browser: `A web3 browser which could read and parse the "cyfs://" protocal.`



### 1.2.1 Below show how to install CodeDao by CYFS browser:

Install CYFS Browser:

|  system   | url  |
|  ----  | ----  |
| CYFS Browser(MAC x86)  | https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/version/production/cyfs-browser-1.0.0.1024-x86.dmg	 |
| CYFS Browser(MAC M1/M2)  | https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/version/production/cyfs-browser-1.0.0.1024-aarch64.dmg |
| CYFS Browser(Windows)	  | https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/version/production/Cyfs_Browser_1.0.0.283-nightly.exe	 |



Open CYFS Browser

![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser.jpg)

In first open, you need to activated the CYFS Browser. below is a successful result.

![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser2.jpg)


After activated your Browser, you can goto Decapp management page.

![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser3.jpg)

In the DecApp Store page, you can click this to add the CodeDao to the list.

![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser4.jpg)

And then, input the CodeDao url: `cyfs://5r4MYfFPKMeHa1fec7dHKmBfowySBfVFvRQvKB956dnF/9tGpLNnYywrCAWoCcyhAcLZtrQpDZtRAg3ai2w47aap2`

Install the newest version of CodeDao app
![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser5.jpg)

Ok Now, you realy close this. After install, you can visit the DecApp's page in the index.

![image.png](https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/document/cyfs%20browser6.jpg)


<!-- --- -->
<!-- `ps: 需要注意的是，每个版本的DEC App都有各自对应的service和页面。你可以通过切换版本来访问对应的页面。可以访问历史版本，这是 CYFS DEC APP的一个特性。不过对应CYFS Git来说，默认使用最新的版本即可。` -->

## 1.3 Install the git-remote-cyfs
So far, you can view other users or repositories in the DecApp, but if you want to push/fetch the code, you also need install the `git-remote-cyfs`   

`git-remote-cyfs` is a git-remote-helper which is a git-extension tool. it can make people use private protocal(outside of http, https, ssh) to use git cmd.

Currently, `git-remote-cyfs` only had the binary cmd. The install gui is no complete yeah.


Let's see how to install it directly:

#### Windows
```
doweload the installer tool
 https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/git-remote-cyfs/installer.exe
```
Exec the installer tool after doweload.
And input the `git-remote-cyfs` terminal, you can see the `git-remote-cyfs` stdout message.


#### Linux
doweload the binary cmd file：
 `https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/git-remote-cyfs/x86_64-unknown-linux-gnu/git-remote-cyfs`  

and copy this file below a PATH, like `/usr/local/bin`  
After copy , git-remote-cyfs is in  `/usr/local/bin/git-remote-cyfs`  
And make sure is executable  `chmod +x /usr/local/bin/git-remote-cyfs`  




#### MacOs
doweload the binary cmd file：

M1  url: `https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/git-remote-cyfs/aarch64-apple-darwin/git-remote-cyfs`    
x86 url: `https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/git-remote-cyfs/x86_64-apple-darwin/git-remote-cyfs`  

and copy this file below a PATH, like `/usr/local/bin`  
After copy , git-remote-cyfs is in  `/usr/local/bin/git-remote-cyfs`  
And make sure is executable  `chmod +x /usr/local/bin/git-remote-cyfs`  





## 1.4 Other requiment
Git


