use crate::layers::Layer;
use anyhow::Result;

pub fn run(layer: &dyn Layer, name: &str, url: &str) -> Result<()> {
    layer.add_remote(name, url)?;
    println!("Added remote '{}' → {}", name, url);

    Ok(())
}
