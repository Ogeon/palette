#[cfg(feature = "phf_codegen")]
extern crate phf_codegen;

mod named;

fn main() {
    named::build();
}