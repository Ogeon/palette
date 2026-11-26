use anyhow::Result;

mod array_cast;
mod codegen_file;
mod from_color_unclamped;
mod lut;
mod named;
mod with_alpha;

static STEPS: &[(fn() -> Result<()>, &str)] = &[
    (named::generate, "Named color constants"),
    (lut::generate, "Conversion lookup tables"),
    (with_alpha::generate, "`WithAlpha` implementations"),
    (array_cast::generate, "`ArrayCast` implementations"),
    (
        from_color_unclamped::generate,
        "`FromColorUnclamped` implementations",
    ),
];

fn main() -> Result<()> {
    let total = STEPS.len();

    for (index, (generate, description)) in STEPS.iter().enumerate() {
        let step_number = index + 1;
        println!("[{step_number}/{total}] {description}");
        generate()?;
    }

    Ok(())
}
