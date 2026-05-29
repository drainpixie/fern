use crate::layers::Layer;
use anyhow::Result;

pub fn run(layer: &dyn Layer) -> Result<()> {
    layer.init()?;
    println!("Initialised repository with Fern");

    Ok(())
}
