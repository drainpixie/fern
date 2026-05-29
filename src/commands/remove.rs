use std::io::{self, Write};

use crate::layers::Layer;
use anyhow::Result;

pub fn run(layer: &dyn Layer, name: &str, yes: bool) -> Result<()> {
    if !yes && !confirm(&format!("Remove remote '{name}'?"))? {
        println!("Aborted");
        return Ok(());
    }

    layer.remove_remote(name)?;
    println!("Removed remote '{name}'");
    Ok(())
}

fn confirm(prompt: &str) -> Result<bool> {
    print!("{prompt} [y/N] ");

    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(matches!(
        buf.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}
