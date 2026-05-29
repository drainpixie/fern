pub mod git;
pub use git::Git;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchTracking {
    pub local: String,
    pub upstream: String,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteStatus {
    pub remote: Remote,
    pub branches: Vec<BranchTracking>,
}

pub trait Layer {
    fn init(&self) -> Result<()>;
    fn remotes(&self) -> Result<Vec<Remote>>;
    fn current_branch(&self) -> Result<String>;
    fn add_remote(&self, name: &str, url: &str) -> Result<()>;
    fn remove_remote(&self, name: &str) -> Result<()>;
    fn rename_remote(&self, old: &str, new: &str) -> Result<()>;
    fn push(&self, remotes: &[String]) -> Result<()>;
    fn pull(&self, remotes: &[String]) -> Result<()>;
    fn fetch(&self, remotes: &[String]) -> Result<()>;
    fn status(&self) -> Result<Vec<RemoteStatus>>;
}
