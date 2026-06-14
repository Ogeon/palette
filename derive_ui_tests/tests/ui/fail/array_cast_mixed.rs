use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(C)]
struct StructTest {
    a: f32,
    b: u32,
}

#[derive(ArrayCast)]
#[repr(C)]
struct GenericStructTest<T> {
    a: f32,
    b: T,
}

fn main() {}
