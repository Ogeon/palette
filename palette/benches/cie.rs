use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use palette::convert::FromColorUnclamped;
use palette::{Lab, Lch, Xyz, Yxy};

#[path = "../tests/convert/data_color_mine.rs"]
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
    let colormine: Vec<ColorMine> = load_data();
    let lab: Vec<Lab> = colormine
        .iter()
        .map(|x| Lab::from_color_unclamped(x.xyz))
        .collect();
    let lch: Vec<Lch> = colormine
        .iter()
        .map(|x| Lch::from_color_unclamped(x.xyz))
        .collect();

    group.throughput(Throughput::Elements(colormine.len() as u64));

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

criterion_group!(benches, cie_conversion);
criterion_main!(benches);
