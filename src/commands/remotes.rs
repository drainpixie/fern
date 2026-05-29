use crate::{cli::Format, layers::Layer, out};
use anyhow::Result;

pub fn run(layer: &dyn Layer, format: &Format) -> Result<()> {
    let remotes = layer.remotes()?;
    out::print_remotes(&remotes, format);

    Ok(())
}
