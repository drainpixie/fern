use crate::layers::Layer;
use anyhow::Result;

pub fn run(layer: &dyn Layer, old: &str, new: &str) -> Result<()> {
    layer.rename_remote(old, new)?;
    println!("Renamed '{old}' → '{new}'");

    Ok(())
}
