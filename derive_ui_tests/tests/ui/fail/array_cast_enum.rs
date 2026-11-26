use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(C)]
enum ArrayCastTest {
    Test(f32),
}

fn main() {}
