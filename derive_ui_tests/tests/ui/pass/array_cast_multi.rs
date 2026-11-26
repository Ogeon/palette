use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(C)]
struct TupleTest(f32, f32);

#[derive(ArrayCast)]
#[repr(C)]
struct GenericTupleTest<T>(T, T);

#[derive(ArrayCast)]
#[repr(C)]
struct StructTest {
    a: f32,
    b: f32,
}

#[derive(ArrayCast)]
#[repr(C)]
struct GenericStructTest<T> {
    a: T,
    b: T,
}

fn main() {}
