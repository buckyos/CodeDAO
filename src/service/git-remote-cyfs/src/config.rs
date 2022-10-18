use std::path::PathBuf;
use cyfs_git_base::*;
use directories::UserDirs;
use std::fs::File;
use std::io::{Write, Read};
use std::str::FromStr;


pub struct Config {
    pub path: PathBuf,
}


impl Config {
    pub fn new() -> Self{
        let user_dirs = {
            let dir = UserDirs::new();
            if dir.is_none() {
                eprintln!("git-remote-cyfs failed to init");
                std::process::exit(1);
            }
            dir.unwrap()
        };
    
        let cyfs_git_home_dir = user_dirs.home_dir().join(".cyfs_git");
        if !cyfs_git_home_dir.exists() {
            std::fs::create_dir_all(cyfs_git_home_dir.clone()).unwrap();
        }
        let config_file = cyfs_git_home_dir.join("remote_config.toml");

        Self{
            path: config_file,
        }
    }

    pub fn init(&self) {
        if !self.path.exists() {
            let content = ChanneConfig::content(ChanneConfig::Nightly);

            let mut file = File::create(self.path.clone()).expect("create config file failed");
            file.write_all(content.as_bytes()).expect("update config file failed");
        }
    }

    pub fn read(&self)  {
        let mut file = File::open(self.path.clone()).expect("open config file failed");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("read config file failed");
        // println!("config: {}", contents);
        ConfigManager::new_oncecell_with_content(&contents);
    }


    pub fn switch_channel(&self, channel: String) {
        let value = ChanneConfig::from_str(&channel).expect("error channel value");
        let content = ChanneConfig::content(value);

        let mut file = File::create(self.path.clone()).expect("change config file failed");
        file.write_all(content.as_bytes()).expect("update config file failed");


        eprintln!("git-remote-cyfs switch channel => {}", channel);
        // write config 
    }
}

enum ChanneConfig {
    DevTest,
    Nightly,
}

impl FromStr for ChanneConfig {
    type Err = ();

    fn from_str(input: &str) -> Result<ChanneConfig, Self::Err> {
        match input {
            "devtest"  => Ok(ChanneConfig::DevTest),
            "nightly"  => Ok(ChanneConfig::Nightly),
            _      => {
                eprintln!("error channel value");
                Err(())
            },
        }
    }
}

impl ChanneConfig {
    fn content(channel: ChanneConfig) -> String {
        let result = match channel {
            ChanneConfig::DevTest => r#"
[main]
channel="dev-test"
deploy_owner_id="5r4MYfFQz9iEzjwHUpc79CwrvXqsh7xUzynkiTUEckxB"
public_service_ood="5aSixgLoakUWDzRq3j4EL5rWfqyKkz5tmm2in517zpCf""#,
            ChanneConfig::Nightly => r#"
[main]
channel="nightly"
deploy_owner_id="5r4MYfFPKMeHa1fec7dHKmBfowySBfVFvRQvKB956dnF"
public_service_ood="5aSixgM3QNdxLzBVoAtsKiucZhwPwAgrprvp5XhEHdoq""#,
        };

        result.to_string()
    }
}

