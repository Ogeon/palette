use palette::cast::ArrayCast;

#[derive(ArrayCast)]
#[repr(C)]
union ArrayCastTest {
    float: f32,
    int: u32,
}

fn main() {}
