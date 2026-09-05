use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(transparent)]
struct TupleTest(f32);

#[derive(ArrayCast)]
#[repr(transparent)]
struct GenericTupleTest<T>(T);

#[derive(ArrayCast)]
#[repr(transparent)]
struct StructTest {
    a: f32,
}

#[derive(ArrayCast)]
#[repr(transparent)]
struct GenericStructTest<T> {
    a: T,
}

fn main() {}
