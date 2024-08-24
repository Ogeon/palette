use anyhow::{Context, Result};

mod codegen_file;
mod named;

fn main() -> Result<()> {
    named::generate().context("could not generate named color constants")?;

    Ok(())
}
