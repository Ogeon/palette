# Contributing

All sorts of contributions are welcome, no matter how large or small. Just fork the repository, implement your change and make a pull request. Don't worry if you are not sure how to implement the change, or if it's not yet done. You can still make a work-in-progress pull request where we can discuss it.

Have a shiny new feature in store? Remember to explain it thoroughly and provide motivation for why it should be added. This makes it easier to review and for everyone to follow your reasoning.

## Testing

Every pull request is automatically tested with continuous integration to deny warnings and any missing documentation. The test suite will thoroughly test each feature separately, but it's a good idea to have ran your local tests with `RUSTFLAGS="-D warnings" cargo test -p palette -p integration_tests --all-features` and also to run `cargo clippy` before submitting your changes. This will make sure that there are no warnings or missing documentation, all for the benefit of the user. Visual Studio Code users can make use of the settings in the `.vscode` folder. They set the default check command and Cargo features, among other things.

There are a number of programs in the `examples` directory that demonstrate applications of the library in more "real" code. The output of these should be checked to see if they are affected by changes made.

### Unit Tests

New features should include appropriate unit tests to prevent future bugs. This is especially important when it comes to dynamic things, like proper conversion or data validation. The unit tests should be placed in a `test` module located within the same module as the code being tested. For example:

```rust
struct Person {
    coolness: f32,
}

impl Person {
    pub fn is_cool(&self) -> bool {
        self.coolness >= 1.0
    }
}

//...

#[cfg(test)]
mod test {
    use super::Person;

    #[test]
    fn is_cool() {
        //...
    }
}
```

Pull requests track the test coverage, but it's not a hard requirement for acceptance. More of a reminder of missing test cases.

### Regression Tests

Each time a bug is fixed, a test of some sort (most likely a unit test) should be added to check that the reported bug has been fixed in the reported use case. This is to prevent the bug from reappearing in the future. The test case may, of course, be expanded to check for more than just the reported case.

## Commits

Commits should be reasonably small and as self contained as possible. If there are fixup commits\* in a pull request, after review and eventual corrections, you will usually be asked to squash them into other commits.

The commit messages themselves don't need to have any particular formatting or syntax (except English). Just make them short, _descriptive_ (!), and tidy.

Not like this:

* fix
* wip
* wip 2

but rather like this:

* Add the missing saturation validation in Hsv
* Make Color implement the Mix trait

Notice how they are written as if they were instructions. They are usually not to written in past tense.

\* Fixup commits are any commits that fix mistakes in other commits within the same pull request. Squashing them into the "original" commits makes the history easier to follow.

## Pull Requests

The header/title of a pull request (PR) should follow the same rules as for commit messages. It should be short and describe the changes as well as possible. The PR description (a.k.a. initial comment, depending on how you view it) should contain a relatively detailed description of the changes. Someone who doesn't know what's going on should be able to look at it and understand what has changed. No secrets or surprises, even if they may be fun.

Both the title and the description will be embedded in the merge commit, and also used as source material for compiling release notes, so it's greatly appreciated if they are as clear and descriptive as possible.

Pull requests that close issues need to mention it in the description. A closed issue should be mentioned as "fixes #123", "closes #123", or [something similar][closing_commits]. This closes the issues automatically when the pull request is merged.

Pull requests that break backwards compatibility should say so in the end of the description, to make sure it's easy to find.

You will see a template when opening a pull request. Just uncomment the parts you need and remove the rest.

[closing_commits]: https://docs.github.com/en/free-pro-team@latest/github/managing-your-work-on-github/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword

## Code Style

The code style is whatever `rustfmt` produces. Running `rustfmt` or `cargo fmt` before committing, or while editing, is therefore strongly recommended. `rustfmt` will typically take care of line wrapping, but in cases where it can't, the recommended line length is somewhere around 80 to 120 characters. Try to prioritize readability and break up any complex expressions that complicate formatting.

### Documentation

There are lints in place to make documentation a requirement. Remember to keep beginners in mind and add examples wherever possible.

Documentation comments are usually capped to 80 characters for source readability. This can be done automatically in some editors like SublimeText's Alt+Q, or via plugins like [Rewrap][rewrap] for Visual Studio Code. Some editors allow for visual rulers to indicate an 80 character width.

[rewrap]: https://marketplace.visualstudio.com/items?itemName=stkb.rewrap

## Adding a Color Type

Color types have grown in size and complexity since this library was first created. It's usually easiest to look at an existing color type that's similar to the new one, and implement the same traits and methods. Here's a set of guidelines for how to implement a color type and what's recommended to add.

### Naming

Try to use the color space's typical name, but also follow Rust's naming convention. For example RGB becomes `Rgb`, with the first letter of the acronym capitalized. xyY, however, became `Yxy` to avoid capitalizing the x and keeping th Y capitalized. The name should also be globally unique, if possible. For example `Okhsl`, rather than just `Hsl` that would collide with the more common, RGB based HSL. Type names should be clear, but reasonably short.

Component/channel names should be spelled out, if possible, since these names can prioritize clear text over brevity. Such as `red` instead of just `r`. In some cases, such as in XYZ, there are no "full names".

### The Type

Most color types are parametric over their component type and a meta type. The meta type may be a white point or some sort of standard. Some color types, such as `Xyz` has a meta type for convenience, even though it's white point agnostic. The meta parameter should be wrapped in `PhantomData`. The properties should be in the same order as the type name suggests. For example, if the type name is `Abc`, the order is `a`, `b`, then `c`.

Color types are also `#[repr(C)]` or `#[repr(transparent)]`, so they can be cast to arrays. More on this later.

An example of a color without a hue may look like this:

```rust
#[repr(C)]
struct MyColor<Wp, T> {
    a: T,
    b: T,
    c: T,
    white_point: PhantomData<Wp>,
}
```

An example of a color with a hue may look like this:

```rust
#[repr(C)]
struct MyColor<Wp, T> {
    hue: MyHue<T>,
    c: T,
    l: T,
    white_point: PhantomData<Wp>,
}
```

The hue type (`MyHue` in the example) should be added in the `hues.rs` module, if the color needs its own hue. This is not necessary if it's based on an already existing definition of hue.

### Constructors

The set of constructors (`new` methods) differs depending on the type of color space. The input values should be in the same order as the type name suggests. For example, if the type name is `Abc`, the order is `a`, `b`, then `c`.

Colors without a hue:

* `pub const fn new(a: T, b: T, c: T) -> Self` - The main constructor.
* `pub fn from_components((a, b, c): (T, T, T)) -> Self` - Constructs the type from a tuple. This can just call `new` internally.

Colors with a hue:

* `pub fn new<H: Into<MyHue<T>>(hue: H, c: T, l: T) -> Self` - The main constructor, which converts hue values to the hue type. This cannot be `const`, due to the lack of support for `const` traits.
* `pub const fn new_const(hue: MyHue<T>, c: T, l: T) -> Self` - An extra `const` constructor, which takes the hue as an already wrapped value.
* `pub fn from_components<H: Into<MyHue<T>>>((hue, c, l): (H, T, T)) -> Self` - Constructs the type from a tuple and converts the hue. This can just call `new` internally.

### Other Common Methods

* `pub fn into_components(self) -> (T, T, T)` or `pub fn into_components(self) -> (MyHue<T>, T, T)` - The opposite of `from_components`. The output values should be in the same order as the input values are when `from_components` is called.
* `pub fn min_a() -> T` and `pub fn max_a() -> T` - Helper methods for getting the typical minimum and maximum of each component. Some types don't have this defined.

### Standard Traits

The standard library provides a number of useful traits that makes the types easier to work with. Some of them can be derived, but they may need to be implemented manually if the color type has a meta type. The derive macro would otherwise limit the meta type as well:

```rust
#[derive(Clone)] // Will require `Wp` to be Clone!
#[repr(C)]
struct MyColor<Wp, T> {
    a: T,
    b: T,
    c: T,
    white_point: PhantomData<Wp>,
}
```

There are also macros for some traits, since a some of the require a lot of repeating code.

Recommended standard traits for all color types:

* `Clone` and `Copy` - Using `impl_copy_clone!`. May be derived if there's no meta type.
* `Debug` - Fine to derive.
* `PartialEq` and `Eq` - Using `impl_eq!` or `impl_eq_hue!`.
* `Default` - Don't derive if there's a meta type. "Default" and "black" are currently conflated, so no macro available. See [#324].
* `Add` - Using `impl_color_add!`.
* `Sub` - Using `impl_color_sub!`.

Additional traits for colors without hue:

* `Mul` - Using `impl_color_mul!`.
* `Div` - Using `impl_color_div!`.

Colors that are usually packed with different component orderings (such as RGB):

* `From<Packed<O, P>>` and vice versa.

Colors with one single component (such as gray/luma):

* `AsRef<T>`, `AsMut<T>`, `From<T>`, `From<&T>`, `From<&mut T>`, and vise versa - Conversions to and from the bare component value. See `Luma` for reference.

Colors with a hexadecimal representation:

* `LowerHex` - Exclude the `#` or similar sigils.
* `UpperHex` - Exclude the `#` or similar sigils.

[#324]: https://github.com/Ogeon/palette/issues/324

### `palette` Traits

Many of the traits in `palette` are implemented using macros. The recommendation is currently to look at a similar color type and copy the macros for it. Some recommended traits will still require manual implementation:

All color types:

* `ArrayCast` - Derived.
* `FromColorUnclamped` - See _Color Conversion_ below.
* `WithAlpha` - Derived.
* `HasBoolMask`.

For stimulus colors, such as RGB and XYZ:

* `StimulusColor`.

Colors with one single component (such as gray/luma):

* `UintCast`.

### Third Party Traits

Types from `palette` implement traits from some third party crates. Some of them are covered by macros, while some are implemented manually:

* `approx` - Using `impl_eq!` or `impl_eq_hue!`.
* `rand` - Using one of `impl_rand_traits_cartesian!`, `impl_rand_traits_cylinder!`, `impl_rand_traits_hsv_cone!`, `impl_rand_traits_hsl_bicone!`, or `impl_rand_traits_hwb_cone!`, depending on which shape the volume of typically valid colors resembles.
* `bytemuck` - The `Zeroable` and `Pod` traits are implemented manually for types that support them.

### Color Conversion

The central trait for color conversion is `FromColorUnclamped`. This should be derived for the color type to implement all combinations of conversions. The number of conversion combinations grows exponentially, so you don't want to do it by hand. Some manual conversions are also necessary.

#### In The `palette_derive` Crate

Add the type's name to its color group (typically `BASE_COLORS`) in `color_types.rs`. This includes it in the list of possible derived conversions. The `preferred_source` tells the library how to find a path to the color type. Each color space has a "parent" space that all connects to `Xyz`. For example, `Hwb` connects to `Hsv`, which connects to `Rgb`, which connects to `Xyz`. The derived conversion code will backtrack as far towards `Xyz` as it needs, until it finds a way to convert to your type.

Other special casing may be needed in other parts of the code, depending on the type. This part can be confusing, so feel free to ask!

#### In The `palette` Crate

Derive `FromColorUnclamped` and add a `#[palette(palette_internal)]` (and more parameters) attribute should be added. The `palette_internal` parameter makes the derive macro find the types and modules in `crate::`.

In addition to that, add the following in the attribute:

* `component = "T"` to point out the component type.
* `white_point = "Wp"` and other meta info to point out the white point type, if necessary. See the list in the documentation or follow the error hints.
* `color_group = "group"` if it's not part of the bas group, such as `"cam16"` if it's a CIE CAM16 derivative.
* `skip_derives(Xyz, Hsv, Hsl)` with all color types you want to convert from with manual implementations.

Add manual conversions for at least one color type and list it in `skip_derives`. `Xyz` is assumed if `skip_derives` is omitted. These are the minimum requirements:

* Implement `FromColorUnclamped<Self> for MyType<Wp, T>`, usually as a unit conversion. This is not blanket implemented, to allow the case when it's not a unit conversion.
* Implement `FromColorUnclamped<MyParentType> for MyType<Wp, T>` for converting from the "parent type" this color type is connected to in `PREFERRED_CONVERSION_SOURCE`. The parent type need a `FromColorUnclamped<MyType<Wp, T>>` implementation, too. Also, make sure to mention it in `skip_derives`.

#### Enabling `FromColor` And `TryFromColor`

The `FromColor` and `TryFromColor` (as well as their `Into` counterparts) are blanket implemented for types that implement `Clamp` and `IsWithinBounds`, respectively, using `impl_clamp!` and `impl_is_within_bounds!`. These traits limit the values to the typical ranges for the color space. For example, `Rgb` has its components limited to `0.0..=1.0` if they are `f32` or `f64`. Implementing these traits will also make the color type implement `FromColor` and `TryFromColor`. They are also generally good to add.
