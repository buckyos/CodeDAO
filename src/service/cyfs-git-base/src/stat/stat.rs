


use cyfs_base::BuckyResult;
use tokei::{Config, Languages};
// use std::path::{Path, PathBuf};
use std::path::PathBuf;
use log::*;

pub fn language_statistics(target_path: PathBuf) -> BuckyResult<()> {

    let excluded = &[".git"];
    let config = Config::default();

    let mut languages = Languages::new();
    languages.get_statistics(&[target_path], excluded, &config);
    // let rust = &languages[&LanguageType::Rust];

    let total = languages.total();


    info!("{:#?}", total);
    println!("{:#?}", total);

    Ok(())
}


#[cfg(test)]
mod main_tests {
    use super::*;
    
    #[async_std::test]
    async fn test_language_statistics() {
        let repo_dir= PathBuf::from(r#"E:\app\cyfs-git-rust\"#);
        language_statistics(repo_dir).unwrap();

    }
}