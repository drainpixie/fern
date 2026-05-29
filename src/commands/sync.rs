use crate::layers::Layer;
use anyhow::Result;
use colored::Colorize;

pub enum Op {
    Push,
    Pull,
    Fetch,
}

pub fn run(layer: &dyn Layer, op: Op, remotes: &[String]) -> Result<()> {
    let branch = layer
        .current_branch()
        .unwrap_or_else(|_| "HEAD".to_string());

    match op {
        Op::Push => {
            println!("{} {}", "↑".yellow().bold(), branch.bold());
            layer.push(remotes)
        }
        Op::Pull => {
            println!("{} {}", "↓".cyan().bold(), branch.bold());
            layer.pull(remotes)
        }
        Op::Fetch => layer.fetch(remotes),
    }?;

    println!();
    Ok(())
}
