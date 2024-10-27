use anyhow::{Context, Result};

mod codegen_file;
mod lut;
mod named;

fn main() -> Result<()> {
    named::generate().context("could not generate named color constants")?;
    lut::generate().context("could not generate conversion lookup tables")?;

    Ok(())
}
