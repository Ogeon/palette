use palette::cast::ArrayCast;
#[repr(transparent)]
struct TupleTest(f32);
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for TupleTest {
    type Array = [f32; 1usize];
}
#[repr(transparent)]
struct GenericTupleTest<T>(T);
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericTupleTest<T> {
    type Array = [T; 1usize];
}
#[repr(transparent)]
struct StructTest {
    a: f32,
}
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for StructTest {
    type Array = [f32; 1usize];
}
#[repr(transparent)]
struct GenericStructTest<T> {
    a: T,
}
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericStructTest<T> {
    type Array = [T; 1usize];
}
fn main() {}
