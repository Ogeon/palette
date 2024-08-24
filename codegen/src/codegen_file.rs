use std::{
    fs::File,
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};

use anyhow::{Context, Result};
use proc_macro2::TokenStream;

const HEADER_COMMENT: &str = r#"// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.
"#;

pub struct CodegenFile {
    file: File,
}

impl CodegenFile {
    /// Create a new generated file.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut file =
            File::create(path).with_context(|| format!("could not open or create {path:?}"))?;

        writeln!(file, "{HEADER_COMMENT}")?;

        Ok(Self { file })
    }

    /// Formats and appends the tokens to the output file.
    pub fn append(&mut self, tokens: TokenStream) -> Result<()> {
        // Taken from https://github.com/Michael-F-Bryan/scad-rs/blob/4dbff0c30ce991105f1e649e678d68c2767e894b/crates/codegen/src/pretty_print.rs

        let mut child = Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("unable to start `rustfmt`. Is it installed?")?;

        let mut stdin = child.stdin.take().unwrap();
        write!(stdin, "{tokens}")?;
        stdin.flush()?;
        drop(stdin);

        let Output {
            status,
            stdout,
            stderr,
        } = child.wait_with_output()?;
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);

        if !status.success() {
            eprintln!("---- Stdout ----");
            eprintln!("{stdout}");
            eprintln!("---- Stderr ----");
            eprintln!("{stderr}");
            let code = status.code();
            match code {
                Some(code) => anyhow::bail!("the `rustfmt` command failed with return code {code}"),
                None => anyhow::bail!("the `rustfmt` command failed"),
            }
        }

        writeln!(self.file, "{stdout}")?;

        Ok(())
    }
}
