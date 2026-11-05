use palette::{self as palette_renamed, cast::ArrayCast};
#[repr(transparent)]
#[palette(crate = "palette_renamed")]
struct Test(f32);
#[automatically_derived]
unsafe impl palette_renamed::cast::ArrayCast for Test {
    type Array = [f32; 1usize];
}
fn main() {}
