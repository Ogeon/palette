use palette::cast::ArrayCast;
#[repr(C)]
struct TupleTest(f32, f32);
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for TupleTest {
    type Array = [f32; 2usize];
}
#[repr(C)]
struct GenericTupleTest<T>(T, T);
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericTupleTest<T> {
    type Array = [T; 2usize];
}
#[repr(C)]
struct StructTest {
    a: f32,
    b: f32,
}
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for StructTest {
    type Array = [f32; 2usize];
}
#[repr(C)]
struct GenericStructTest<T> {
    a: T,
    b: T,
}
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericStructTest<T> {
    type Array = [T; 2usize];
}
fn main() {}
