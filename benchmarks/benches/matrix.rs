use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

use palette::encoding;
use palette::matrix::{matrix_inverse, multiply_3x3, rgb_to_xyz_matrix};

fn matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix functions");

    let inp1 = [1.0f32, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0];
    let inp2 = [4.0, 5.0, 6.0, 6.0, 5.0, 4.0, 4.0, 6.0, 5.0];
    let inverse: [f32; 9] = [3.0, 0.0, 2.0, 2.0, 0.0, -2.0, 0.0, 1.0, 1.0];

    group.bench_function("multiply_3x3", |b| {
        b.iter(|| multiply_3x3(black_box(inp1), black_box(inp2)))
    });
    group.bench_with_input("matrix_inverse", &inverse, |b, inverse| {
        b.iter(|| matrix_inverse(*inverse))
    });
    group.bench_function("rgb_to_xyz_matrix", |b| {
        b.iter(rgb_to_xyz_matrix::<encoding::Srgb, f32>)
    });
}

criterion_group!(benches, matrix);
criterion_main!(benches);
