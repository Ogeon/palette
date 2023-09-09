// Checks that issue #283 (fixed in 0.7.1) doesn't re-appear. The cause was the
// existence of `impl Mul<PreAlpha<C>> for f32` and `impl Mul<PreAlpha<C>> for
// f64`.

// Both of these uses are necessary for triggering the issue
#[allow(unused_imports)]
use palette::Oklch;
#[allow(unused_imports)]
use scad::OffsetType;

fn main() {
    println!("{}", 42.0 * 1.0); // bug also happens when specifying f32 or f64
}
