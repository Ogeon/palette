use palette::{self as palette_renamed, cast::ArrayCast};

#[derive(ArrayCast)]
#[repr(transparent)]
#[palette(crate = "palette_renamed")]
struct Test(f32);
fn main() {}
