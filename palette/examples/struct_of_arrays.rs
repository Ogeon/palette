use palette::{cast::ComponentsInto, color_difference::EuclideanDistance, IntoColor, Oklab, Srgb};

fn main() {
    let image = image::open("example-data/input/fruits.png")
        .expect("could not open 'example-data/input/fruits.png'")
        .to_rgb8();

    let image: &[Srgb<u8>] = image.as_raw().components_into();

    // Convert and collect the colors in a struct-of-arrays (SoA) format, where
    // each component is a Vec of all the pixels' component values.
    let oklab_image: Oklab<Vec<f32>> = image
        .iter()
        .map(|rgb| rgb.into_linear::<f32>().into_color())
        .collect();

    // Find the min, max and average of each component. We are doing it by
    // iterating over each component Vec separately, starting with l...
    let (min_l, max_l, average_l) = get_min_max_average(oklab_image.l.iter().copied());
    let (min_a, max_a, average_a) = get_min_max_average(oklab_image.a.iter().copied());
    let (min_b, max_b, average_b) = get_min_max_average(oklab_image.b.iter().copied());

    // Find out how far the colors in the image are from the average color. In
    // this case, we can just iterate over all of the colors and consume the
    // collection(s).
    let average_color = Oklab::new(average_l, average_a, average_b);
    let (min_d, max_d, average_d) = get_min_max_average(
        oklab_image
            .into_iter()
            .map(|color| average_color.distance(color)),
    );

    // Print the stats.
    println!("Oklab l: min {min_l}, average {average_l}, max {max_l}");
    println!("Oklab a: min {min_a}, average {average_a}, max {max_a}");
    println!("Oklab b: min {min_b}, average {average_b}, max {max_b}");
    println!("Distance from average color: min {min_d}, average {average_d}, max {max_d}");
}

/// Calculates the min, max and average of the iterator's values.
fn get_min_max_average(iter: impl ExactSizeIterator<Item = f32>) -> (f32, f32, f32) {
    let length = iter.len();

    let (min, max, sum) = iter.fold(
        (f32::INFINITY, f32::NEG_INFINITY, 0.0),
        |(min, max, sum), value| (min.min(value), max.max(value), sum + value),
    );

    (min, max, sum / length as f32)
}
