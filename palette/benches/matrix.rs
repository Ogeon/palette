use criterion::{black_box, criterion_group, criterion_main, Criterion};

use palette::encoding;
use palette::matrix::{
    matrix_inverse, multiply_3x3, multiply_rgb_to_xyz, multiply_xyz, multiply_xyz_to_rgb,
    rgb_to_xyz_matrix,
};
use palette::white_point::{WhitePoint, D65};
use palette::{LinSrgb, Xyz};

fn matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix functions");

    let inp1 = [0.1, 0.2, 0.3, 0.3, 0.2, 0.1, 0.2, 0.1, 0.3];
    let inp2 = Xyz::new(0.4, 0.6, 0.8);
    let inp3 = [1.0f32, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0];
    let inp4 = [4.0, 5.0, 6.0, 6.0, 5.0, 4.0, 4.0, 6.0, 5.0];
    let inverse: [f32; 9] = [3.0, 0.0, 2.0, 2.0, 0.0, -2.0, 0.0, 1.0, 1.0];
    let color = LinSrgb::new(0.2f32, 0.8, 0.4);
    let mat3 = rgb_to_xyz_matrix::<encoding::Srgb, f32>();
    let wp: Xyz<D65, f32> = D65::get_xyz().with_white_point();

    group.bench_function("multiply_xyz", |b| {
        b.iter(|| multiply_xyz::<_>(black_box(inp1), black_box(inp2)))
    });
    group.bench_function("multiply_xyz_to_rgb", |b| {
        b.iter(|| multiply_xyz_to_rgb::<encoding::Srgb, _, _>(black_box(inp1), black_box(wp)))
    });
    group.bench_function("multiply_rgb_to_xyz", |b| {
        b.iter(|| multiply_rgb_to_xyz(black_box(mat3), black_box(color)))
    });
    group.bench_function("multiply_3x3", |b| {
        b.iter(|| multiply_3x3(black_box(inp3), black_box(inp4)))
    });
    group.bench_with_input("matrix_inverse", &inverse, |b, inverse| {
        b.iter(|| matrix_inverse(*inverse))
    });
    group.bench_function("rgb_to_xyz_matrix", |b| {
        b.iter(|| rgb_to_xyz_matrix::<encoding::Srgb, f32>())
    });
}

criterion_group!(benches, matrix);
criterion_main!(benches);
