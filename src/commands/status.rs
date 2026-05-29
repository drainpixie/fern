use crate::{cli::Format, layers::Layer, out};
use anyhow::Result;

pub fn run(layer: &dyn Layer, format: &Format) -> Result<()> {
    let statuses = layer.status()?;
    out::print_status(&statuses, format);

    Ok(())
}
