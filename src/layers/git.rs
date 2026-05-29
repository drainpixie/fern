use anyhow::{Context, Result, bail};
use std::process::{Command, Stdio};

use super::{BranchTracking, Layer, Remote, RemoteStatus};

#[derive(Default)]
pub struct Git;

impl Git {
    pub fn new() -> Self {
        Self
    }

    fn run(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(args)
            .output()
            .context("failed to spawn git, is it installed and in PATH?")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

            if stderr.is_empty() {
                bail!("git {} failed", args.first().copied().unwrap_or("command"))
            } else {
                bail!("{}", stderr)
            }
        }
    }

    fn run_interactive(&self, args: &[&str]) -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context("failed to spawn git")?;

        if status.success() {
            Ok(())
        } else {
            bail!("git {} failed", args.first().copied().unwrap_or("command"))
        }
    }

    fn rev_count(&self, base: &str, tip: &str) -> Result<usize> {
        self.run(&["rev-list", "--count", &format!("{base}..{tip}")])
            .and_then(|s| s.parse().context("unexpected rev-list output"))
    }

    fn all_remote_names(&self) -> Result<Vec<String>> {
        Ok(self
            .run(&["remote"])?
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }

    fn branches_tracking(&self, remote: &str) -> Result<Vec<BranchTracking>> {
        let out = self.run(&[
            "for-each-ref",
            "--format=%(refname:short)\t%(upstream:short)",
            "refs/heads",
        ])?;

        out.lines()
            .filter(|l| !l.is_empty())
            .filter_map(|line| {
                let (local, upstream) = line.split_once('\t')?;
                upstream
                    .starts_with(&format!("{remote}/"))
                    .then(|| (local.to_string(), upstream.to_string()))
            })
            .map(|(local, upstream)| {
                let ahead = self.rev_count(&upstream, &local).unwrap_or(0);
                let behind = self.rev_count(&local, &upstream).unwrap_or(0);

                Ok(BranchTracking {
                    local,
                    upstream,
                    ahead,
                    behind,
                })
            })
            .collect()
    }

    fn resolve_targets(&self, remotes: &[String]) -> Result<Vec<String>> {
        if remotes.is_empty() {
            self.all_remote_names()
        } else {
            Ok(remotes.to_vec())
        }
    }
}

impl Layer for Git {
    fn init(&self) -> Result<()> {
        self.run_interactive(&["init"])
    }

    fn current_branch(&self) -> Result<String> {
        self.run(&["symbolic-ref", "--short", "HEAD"])
            .context("could not determine current branch, detached HEAD?")
    }

    fn remotes(&self) -> Result<Vec<Remote>> {
        self.all_remote_names()?
            .into_iter()
            .map(|name| {
                let url = self.run(&["remote", "get-url", &name])?;
                Ok(Remote { name, url })
            })
            .collect()
    }

    fn add_remote(&self, name: &str, url: &str) -> Result<()> {
        self.run(&["remote", "add", name, url])?;
        Ok(())
    }

    fn remove_remote(&self, name: &str) -> Result<()> {
        self.run(&["remote", "remove", name])?;
        Ok(())
    }

    fn rename_remote(&self, old: &str, new: &str) -> Result<()> {
        self.run(&["remote", "rename", old, new])?;
        Ok(())
    }

    fn push(&self, remotes: &[String]) -> Result<()> {
        self.resolve_targets(remotes)?
            .iter()
            .try_for_each(|remote| self.run_interactive(&["push", remote]))
    }

    fn pull(&self, remotes: &[String]) -> Result<()> {
        self.resolve_targets(remotes)?
            .iter()
            .try_for_each(|remote| self.run_interactive(&["pull", remote]))
    }

    fn fetch(&self, remotes: &[String]) -> Result<()> {
        self.resolve_targets(remotes)?
            .iter()
            .try_for_each(|remote| self.run_interactive(&["fetch", remote]))
    }

    fn status(&self) -> Result<Vec<RemoteStatus>> {
        self.remotes()?
            .into_iter()
            .map(|remote| {
                let branches = self.branches_tracking(&remote.name)?;
                Ok(RemoteStatus { remote, branches })
            })
            .collect()
    }
}
