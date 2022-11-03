
use std::env::set_var;
use prost_build;


/// 生成 cyfs_git.rs
#[async_std::main]
async fn main() {
    // cwd  <repo>/services/
    // generate target path
    set_var("OUT_DIR", "cyfs-git-base/src/object_type/proto");

    let cwd = std::env::current_dir().unwrap();
    let proto_root_path = cwd
        .join("proto");
    let proto_file_path = proto_root_path.join("cyfs_git.proto");
    println!("path {:?}", proto_file_path);

    let mut config = prost_build::Config::new();
    config.compile_protos(
        &[proto_file_path],
        &[proto_root_path]).unwrap();
}
