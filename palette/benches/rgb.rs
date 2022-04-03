use criterion::{black_box, criterion_group, criterion_main, Criterion};
use palette::convert::FromColorUnclamped;
use palette::encoding;
use palette::{Hsl, Hsv, Hwb, IntoColor, LinSrgb, Srgb};

type SrgbHsv = Hsv<encoding::Srgb>;
type SrgbHsl = Hsl<encoding::Srgb>;
type SrgbHwb = Hwb<encoding::Srgb>;
type LinHsv = Hsv<encoding::Linear<encoding::Srgb>>;
type LinHsl = Hsl<encoding::Linear<encoding::Srgb>>;
type LinHwb = Hwb<encoding::Linear<encoding::Srgb>>;

#[path = "../tests/convert/data_color_mine.rs"]
#[allow(dead_code)]
mod data_color_mine;
use data_color_mine::{load_data, ColorMine};

/* Benches the following conversions:
    - rgb to linear
    - rgb to hsl
    - hsv to hsl
    - rgb to hsv
    - hsl to hsv
    - hwb to hsv
    - hsv to hwb
    - xyz to rgb
    - hsl to rgb
    - hsv to rgb
    - hsv to linear hsv
    - linear hsv to hsv
    - hsl to linear hsl
    - linear hsl to hsl
    - hwb to linear hwb
    - linear hwb to hwb
    - linsrgb to rgb
    - rgb_u8 to linsrgb_f32
    - linsrgb_f32 to rgb_u8
*/

fn rgb_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rgb family");
    let mut colormine: Vec<ColorMine<f32>> = load_data();
    colormine.truncate(colormine.len() - colormine.len() % 8);
    assert_eq!(
        colormine.len() % 8,
        0,
        "number of colors must be a multiple of 8 for a fair comparison with SIMD"
    );
    #[cfg(feature = "wide")]
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

    let rgb_u8: Vec<Srgb<u8>> = colormine.iter().map(|x| x.rgb.into_format()).collect();

    let linear_hsv: Vec<LinHsv> = colormine.iter().map(|x| x.hsv.into_color()).collect();
    let linear_hsl: Vec<LinHsl> = colormine.iter().map(|x| x.hsl.into_color()).collect();
    let linear_hwb: Vec<LinHwb> = colormine.iter().map(|x| x.hwb.into_color()).collect();

    group.throughput(criterion::Throughput::Elements(colormine.len() as u64));
    group.bench_with_input("rgb to linsrgb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(c.rgb.into_linear());
            }
        })
    });
    #[cfg(feature = "wide")]
    group.bench_with_input(
        "rgb to linsrgb - wide::f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(c.rgb.into_linear());
                }
            })
        },
    );
    group.bench_with_input("rgb to hsl", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hsl::from_color_unclamped(c.rgb));
            }
        })
    });
    #[cfg(feature = "wide")]
    group.bench_with_input(
        "rgb to hsl - wide::f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(Hsl::from_color_unclamped(c.rgb));
                }
            })
        },
    );
    group.bench_with_input("hsv to hsl", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hsl::from_color_unclamped(c.hsv));
            }
        })
    });
    group.bench_with_input("rgb to hsv", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hsv::from_color_unclamped(c.rgb));
            }
        })
    });
    #[cfg(feature = "wide")]
    group.bench_with_input(
        "rgb to hsv - wide::f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(Hsv::from_color_unclamped(c.rgb));
                }
            })
        },
    );
    group.bench_with_input("hsl to hsv", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hsv::from_color_unclamped(c.hsl));
            }
        })
    });
    group.bench_with_input("hwb to hsv", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hsv::<encoding::Srgb, _>::from_color_unclamped(c.hwb));
            }
        })
    });
    group.bench_with_input("hsv to hwb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Hwb::from_color_unclamped(c.hsv));
            }
        })
    });
    group.bench_with_input("xyz to linsrgb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(LinSrgb::from_color_unclamped(c.xyz));
            }
        })
    });
    #[cfg(feature = "wide")]
    group.bench_with_input(
        "xyz to linsrgb - wide::f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(LinSrgb::from_color_unclamped(c.xyz));
                }
            })
        },
    );
    group.bench_with_input("hsl to rgb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Srgb::from_color_unclamped(c.hsl));
            }
        })
    });
    group.bench_with_input("hsv to rgb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Srgb::from_color_unclamped(c.hsv));
            }
        })
    });
    group.bench_with_input("hsv to linear hsv", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(LinHsv::from_color_unclamped(c.hsv));
            }
        })
    });
    group.bench_with_input("linear hsv to hsv", &linear_hsv, |b, linear_hsv| {
        b.iter(|| {
            for &c in linear_hsv {
                black_box(SrgbHsv::from_color_unclamped(c));
            }
        })
    });
    group.bench_with_input("hsl to linear hsl", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(LinHsl::from_color_unclamped(c.hsl));
            }
        })
    });
    group.bench_with_input("linear hsl to hsl", &linear_hsl, |b, linear_hsl| {
        b.iter(|| {
            for &c in linear_hsl {
                black_box(SrgbHsl::from_color_unclamped(c));
            }
        })
    });
    group.bench_with_input("hwb to linear hwb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(LinHwb::from_color_unclamped(c.hwb));
            }
        })
    });
    group.bench_with_input("linear hwb to hwb", &linear_hwb, |b, linear_hwb| {
        b.iter(|| {
            for &c in linear_hwb {
                black_box(SrgbHwb::from_color_unclamped(c));
            }
        })
    });
    group.bench_with_input("linsrgb to rgb", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Srgb::from_linear(c.linear_rgb));
            }
        })
    });
    #[cfg(feature = "wide")]
    group.bench_with_input(
        "linsrgb to rgb - wide::f32x8",
        &wide_colormine,
        |b, wide_colormine| {
            b.iter(|| {
                for c in wide_colormine {
                    black_box(Srgb::from_linear(c.linear_rgb));
                }
            })
        },
    );
    group.bench_with_input("rgb_u8 to linsrgb_f32", &rgb_u8, |b, rgb_u8| {
        b.iter(|| {
            for c in rgb_u8 {
                black_box(c.into_format::<f32>().into_linear());
            }
        })
    });
    group.bench_with_input("linsrgb_f32 to rgb_u8", &colormine, |b, colormine| {
        b.iter(|| {
            for c in colormine {
                black_box(Srgb::from_linear(c.linear_rgb).into_format::<u8>());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, rgb_conversion);
criterion_main!(benches);
