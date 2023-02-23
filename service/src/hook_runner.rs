use std::{path::PathBuf};
use anyhow::Result;
use tokio::{
    fs::read_dir,
    process::Command
};
use futures_util::stream::{self,TryStreamExt,StreamExt};

#[derive(Default, Debug)]
pub struct HookRunner {
    hooks: Vec<PathBuf>,
}

impl HookRunner {
    /// Completes when ALL of the hooks have finished processing
    pub async fn run_hooks(&self, url: &str) {
        let command_iter = self.hooks
            .iter()
            .cloned()
            .map(|x| {
                let mut command = Command::new(x);
                command.env("KANTYNA_LASER_URL", url);
                command
            });
        stream::iter(command_iter)
            .then(|mut cmd| async move {
                cmd.spawn()
            })
            .map_err(|e| {
                eprintln!("Could not spawn hook process: {}",e);
            })
            .filter_map(|g| async move {
                g.ok()
            }).for_each_concurrent(None, |mut child| async move {
                match child.wait().await {
                    Ok(status) => {
                        if ! status.success() {
                            eprintln!("A hook exited with an error status: {:?}",status);
                        }
                    },
                    Err(e) => {
                        eprintln!("Could not wait for hook child process: {}",e);
                    }
                }
            }).await;
    }
    /// Load hooks from the hook_dir
    pub async fn load(&mut self) -> Result<()> {
        self.hooks.clear();
        let hook_dir = std::env::var("KANTYNA_LASER_HOOK_DIR").unwrap_or("./hooks".to_owned());
        eprintln!("Loading hooks from '{}'",&hook_dir);
        let mut dir_iter = read_dir(&hook_dir).await?;
        while let Ok(Some(entry)) = dir_iter.next_entry().await {
            if ! entry.file_type().await?.is_dir() {
                let hook_path = entry.path();
                eprintln!("Found hook: '{}'",&hook_path.to_str().unwrap());
                self.hooks.push(hook_path);
            }
        }
        eprintln!("Done loading hooks.");
        Ok(())
    }
}