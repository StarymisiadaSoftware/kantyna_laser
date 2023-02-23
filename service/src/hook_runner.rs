use std::path::PathBuf;
use anyhow::Result;

#[derive(Default, Debug)]
pub struct HookRunner {
    hooks: Vec<PathBuf>,
}

impl HookRunner {
    /// Completes when ALL of the hooks have finished processing
    pub async fn run_hooks(&self) {}
    /// Load hooks from the hook_dir
    pub async fn load(&mut self) -> Result<()> {
        Ok(())
    }
}