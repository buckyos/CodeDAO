#![windows_subsystem = "windows"]
use winreg::types::{FromRegValue, ToRegValue};
use winreg::{enums::*, RegKey};
use directories::UserDirs;
use std::io::Cursor;
use std::path::PathBuf;

#[tokio::main]
async fn main()  {
    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();

    let cyfs_git_home_dir = user_dirs.home_dir().join(".cyfs_git");
    println!("{:?}", cyfs_git_home_dir);
    if !cyfs_git_home_dir.exists() {
        println!("create dir {:?}", cyfs_git_home_dir);
        std::fs::create_dir_all(cyfs_git_home_dir.clone()).unwrap();
    }

    download(cyfs_git_home_dir.clone()).await;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions

    let old_path_value = env.get_raw_value("Path").expect("get env path");
    let value_type = old_path_value.vtype.clone();
    let old_path_value = String::from_reg_value(&old_path_value).unwrap();
    let cyfs_git_bin_dir = String::from(cyfs_git_home_dir.to_string_lossy());

    let mut paths: Vec<&str> = old_path_value.split(";").filter(|p| !p.is_empty()).collect();
    let r = paths.iter().find(|p| p == &&&cyfs_git_bin_dir);
    if r.is_some() {
        println!("bin path is already in system env path , don't need to insert");
        std::process::exit(0);
    }

    paths.push(cyfs_git_bin_dir.as_str());
    paths.push("");
    let new_path_value = paths.join(";");
    
    let mut new_path_value = new_path_value.to_reg_value();
    new_path_value.vtype = value_type;
    env.set_raw_value("Path", &new_path_value).unwrap();
}


async fn download(cyfs_git_home_dir: PathBuf) {
    // download file 
    let target = "https://cyfs-admin.oss-cn-shenzhen.aliyuncs.com/git-remote-cyfs/x86_64-pc-windows-msvc/git-remote-cyfs.exe";
    let file_name = cyfs_git_home_dir.join("git-remote-cyfs.exe");
    
    let response = reqwest::get(target).await.expect("request failed");
    let mut file = std::fs::File::create(file_name).unwrap();
    let mut content =  Cursor::new(response.bytes().await.expect("body invalid"));
    std::io::copy(&mut content, &mut file).expect("copy file failed");
    println!("download binary ok");

}