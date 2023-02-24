use crate::consts::*;
use anyhow::{Context, Result};
use futures_util::stream::{self, StreamExt, TryStreamExt};
use std::{path::PathBuf, process::Stdio};
use tokio::{fs::read_dir, process::Command};

#[derive(Default, Debug)]
pub struct HookRunner {
    hooks: Vec<PathBuf>,
}

impl HookRunner {
    /// Completes when ALL of the hooks have finished processing
    pub async fn run_hooks(&self, url: &str) {
        eprintln!("Running hooks for song: '{}'.", url);
        let command_iter = self.hooks.iter().cloned().map(|x| {
            let mut command = Command::new(&x);
            command.env(HOOK_URL_ENVVAR, url);
            command.stdin(Stdio::null());
            //command.stdout(Stdio::null());
            (command, x)
        });
        stream::iter(command_iter)
            .then(|(mut cmd, hook_path)| async move {
                cmd.spawn()
                    .context(format!("Hook path: '{}'", hook_path.to_str().unwrap()))
                    .map(|good| (good, hook_path))
            })
            .map_err(|e| {
                eprintln!("Could not spawn hook process: {:#?}", e);
            })
            .filter_map(|g| async move { g.ok() })
            .for_each_concurrent(None, |(mut child, hook_path)| async move {
                match child.wait().await {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!(
                                "Hook '{}' exited with an error status: {:?}",
                                hook_path.to_str().unwrap(),
                                status
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Could not wait for child process of hook '{}'. Error: {}",
                            hook_path.to_str().unwrap(),
                            e
                        );
                    }
                }
            })
            .await;
        eprintln!("Done running hooks for song: '{}'.", url);
    }
    /// Load hooks from the hook_dir
    pub async fn load(&mut self) -> Result<()> {
        self.hooks.clear();
        let hook_dir = std::env::var(HOOK_DIR_ENVVAR).unwrap_or("./hooks".to_owned());
        eprintln!("Loading hooks from '{}'", &hook_dir);
        let mut dir_iter = read_dir(&hook_dir).await?;
        while let Ok(Some(entry)) = dir_iter.next_entry().await {
            if !entry.file_type().await?.is_dir() {
                let hook_path = entry.path();
                eprintln!("Found hook: '{}'", &hook_path.to_str().unwrap());
                self.hooks.push(hook_path);
            }
        }
        eprintln!("Done loading hooks.");
        Ok(())
    }
}
