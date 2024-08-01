use std::path::Path;

use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use palette::{
    color_difference::{Ciede2000, DeltaE, ImprovedDeltaE},
    convert::{Convert, FromColorUnclamped},
};
use palette::{Lab, Lch, Xyz, Yxy};

#[path = "../../integration_tests/tests/convert/data_color_mine.rs"]
#[allow(dead_code)]
mod data_color_mine;
use data_color_mine::{load_data, ColorMine};

/* Benches the following conversions:
    - xyz to lab
    - lch to lab
    - lab to lch
    - rgb to xyz
    - yxy to xyz
    - lab to xyz
    - xyz to yxy
*/

fn cie_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cie family");
    let mut colormine: Vec<ColorMine<f32>> = load_data(Some(Path::new(
        "../integration_tests/tests/convert/data_color_mine.csv",
    )));
    colormine.truncate(colormine.len() - colormine.len() % 8);
    assert_eq!(
        colormine.len() % 8,
        0,
        "number of colors must be a multiple of 8 for a fair comparison with SIMD"
    );

    let wide_colormine: Vec<_> = colormine
        .chunks_exact(8)
        .map(|chunk| {
            ColorMine::<wide::f32x8>::from([
                chunk[0].clone(),
                chunk[1].clone(),
                chunk[2].clone(),
                chunk[3].clone(),
                chunk[4].clone(),
                chunk[5].clone(),
                chunk[6].clone(),
                chunk[7].clone(),
            ])
        })
        .collect();

    let lab: Vec<Lab> = colormine
        .iter()
        .map(|x| Lab::from_color_unclamped(x.xyz))
        .collect();
    let lch: Vec<Lch> = colormine
        .iter()
        .map(|x| Lch::from_color_unclamped(x.xyz))
        .collect();

    let rgb_to_xyz_matrix = Xyz::matrix_from_rgb();

    group.bench_with_input("xyz to lab", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Lab::from_color_unclamped(c.xyz));
            }
        })
    });
    group.bench_with_input("lch to lab", &lch, |b, lch| {
        b.iter(|| {
            for c in lch {
                black_box(Lab::from_color_unclamped(*c));
            }
        })
    });
    group.bench_with_input("lab to lch", &lab, |b, lab| {
        b.iter(|| {
            for c in lab {
                black_box(Lch::from_color_unclamped(*c));
            }
        })
    });
    group.bench_with_input("linsrgb to xyz", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Xyz::from_color_unclamped(c.linear_rgb));
            }
        })
    });
    group.bench_with_input(
        "linsrgb to xyz - wide f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(Xyz::from_color_unclamped(c.linear_rgb));
                }
            })
        },
    );
    group.bench_with_input(
        "linsrgb to xyz - Matrix3",
        &(&colormine, rgb_to_xyz_matrix),
        |b, &(colormine, matrix)| {
            b.iter(|| {
                for c in colormine {
                    black_box(matrix.convert(c.linear_rgb));
                }
            })
        },
    );
    group.bench_with_input("yxy to xyz", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Xyz::from_color_unclamped(c.yxy));
            }
        })
    });
    group.bench_with_input("lab to xyz", &lab, |b, lab| {
        b.iter(|| {
            for c in lab {
                black_box(Xyz::from_color_unclamped(*c));
            }
        })
    });
    group.bench_with_input("xyz to yxy", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Yxy::from_color_unclamped(c.xyz));
            }
        })
    });

    group.finish();
}

fn cie_delta_e(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cie delta E");
    let colormine: Vec<ColorMine<f32>> = load_data(Some(Path::new(
        "../integration_tests/tests/convert/data_color_mine.csv",
    )));

    let lab: Vec<Lab> = colormine
        .iter()
        .map(|x| Lab::from_color_unclamped(x.xyz))
        .collect();
    let lch: Vec<Lch> = colormine
        .iter()
        .map(|x| Lch::from_color_unclamped(x.xyz))
        .collect();

    group.bench_with_input("Lab delta E", &lab, |b, lab| {
        b.iter(|| {
            for &lhs in lab {
                for &rhs in lab.iter().rev() {
                    black_box(lhs.delta_e(rhs));
                }
            }
        })
    });

    group.bench_with_input("Lch delta E", &lch, |b, lch| {
        b.iter(|| {
            for &lhs in lch {
                for &rhs in lch.iter().rev() {
                    black_box(lhs.delta_e(rhs));
                }
            }
        })
    });

    group.bench_with_input("Lch delta E via Lab", &lch, |b, lch| {
        b.iter(|| {
            for &lhs in lch {
                for &rhs in lch.iter().rev() {
                    black_box(
                        Lab::from_color_unclamped(lhs).delta_e(Lab::from_color_unclamped(rhs)),
                    );
                }
            }
        })
    });

    group.bench_with_input("Lab improved delta E", &lab, |b, lab| {
        b.iter(|| {
            for &lhs in lab {
                for &rhs in lab.iter().rev() {
                    black_box(lhs.improved_delta_e(rhs));
                }
            }
        })
    });

    group.bench_with_input("Lab CIEDE2000", &lab, |b, lab| {
        b.iter(|| {
            for &lhs in lab {
                for &rhs in lab.iter().rev() {
                    black_box(lhs.difference(rhs));
                }
            }
        })
    });

    group.bench_with_input("Lch CIEDE2000", &lch, |b, lch| {
        b.iter(|| {
            for &lhs in lch {
                for &rhs in lch.iter().rev() {
                    black_box(lhs.difference(rhs));
                }
            }
        })
    });
}

criterion_group!(benches, cie_conversion, cie_delta_e);
criterion_main!(benches);
