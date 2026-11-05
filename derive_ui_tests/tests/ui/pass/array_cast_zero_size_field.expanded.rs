use palette::cast::ArrayCast;
#[repr(C)]
struct TupleTest(f32, #[palette(unsafe_zero_sized)] std::marker::PhantomData<()>);
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for TupleTest {
    type Array = [f32; 1usize];
}
#[repr(C)]
struct GenericTupleTest<T>(
    T,
    #[palette(unsafe_zero_sized)]
    std::marker::PhantomData<()>,
);
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericTupleTest<T> {
    type Array = [T; 1usize];
}
#[repr(C)]
struct StructTest {
    a: f32,
    #[palette(unsafe_zero_sized)]
    b: std::marker::PhantomData<()>,
}
#[automatically_derived]
unsafe impl palette::cast::ArrayCast for StructTest {
    type Array = [f32; 1usize];
}
#[repr(C)]
struct GenericStructTest<T> {
    a: T,
    #[palette(unsafe_zero_sized)]
    b: std::marker::PhantomData<()>,
}
#[automatically_derived]
unsafe impl<T> palette::cast::ArrayCast for GenericStructTest<T> {
    type Array = [T; 1usize];
}
fn main() {}
