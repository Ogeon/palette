use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(C)]
struct TupleTest(
    f32,
    #[palette(unsafe_zero_sized)] std::marker::PhantomData<()>,
);

#[derive(ArrayCast)]
#[repr(C)]
struct GenericTupleTest<T>(
    T,
    #[palette(unsafe_zero_sized)] std::marker::PhantomData<()>,
);

#[derive(ArrayCast)]
#[repr(C)]
struct StructTest {
    a: f32,
    #[palette(unsafe_zero_sized)]
    b: std::marker::PhantomData<()>,
}

#[derive(ArrayCast)]
#[repr(C)]
struct GenericStructTest<T> {
    a: T,
    #[palette(unsafe_zero_sized)]
    b: std::marker::PhantomData<()>,
}

fn main() {}
