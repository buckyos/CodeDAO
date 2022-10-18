use std::time::Duration;
use std::process::{Command, Stdio};
use std::path::PathBuf;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
pub struct RuntimeLauncher;

trait RuntimeExtOption {
    fn ext_option(&mut self) -> &mut Self;
}

impl RuntimeExtOption for Command {
    #[cfg(windows)]
    fn ext_option(&mut self) -> &mut Self {
        self.creation_flags(0x08000000)
    }

    #[cfg(not(windows))]
    fn ext_option(&mut self) -> &mut Self {
        self
    }
}

impl RuntimeLauncher {
    pub async fn launch() {
        async_std::task::spawn(async {
            loop {
                if !cyfs_util::process::check_process_mutex(cyfs_base::CYFS_RUNTIME_NAME) {
                    let runtime_path = Self::runtime_path();
                    log::info!("launch {}", runtime_path.to_string_lossy().to_string());
                    let _ = Command::new(runtime_path.as_path())
                        .ext_option()
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .stdin(Stdio::null())
                        .arg("--proxy-port=38090")
                        .spawn()
                        .map_err(|e| {
                        log::info!("launch {} failed.err{}", runtime_path.to_string_lossy().to_string(), e);
                    });
                }
                async_std::task::sleep(Duration::new(1, 0)).await;
            }
        });
    }

    #[cfg(windows)]
    pub fn runtime_path() -> PathBuf {
        let cyfs_path = dirs::config_dir().unwrap().join("cyfs");
        cyfs_path.join("services").join("runtime").join("cyfs-runtime.exe")
    }

    #[cfg(not(windows))]
    pub fn runtime_path() -> PathBuf {
        let cyfs_path = dirs::config_dir().unwrap().join("cyfs");
        cyfs_path.join("services").join("runtime").join("cyfs-runtime")
    }
}
