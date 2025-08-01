# palette_math

The low level color math behind the more high level [`palette`](https://crates.io/crates/palette) crate. This create's primary purpose is to facilitate `palette`, but its content is also useable on its own.

## Minimum Supported Rust Version (MSRV)

This version of Palette has been automatically tested with Rust version `1.61.0`, `1.63.0`, and the `stable`, `beta`, and `nightly` channels. The minimum supported version may vary with the set of enabled features.

Future versions of the library may advance the minimum supported version to make use of new language features, but this will normally be considered a breaking change. Exceptions may be made for security patches, dependencies advancing their MSRV in minor or patch releases, and similar changes.

## Getting Started

Add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
palette_math = "0.7.6"
```

or this if you want to opt out of `std`:

```toml
# Uses libm instead of std for floating point math:
palette_math = { version = "0.7.6", features = ["libm"], default-features = false }
```

### Cargo Features

These features are enabled by default:

* `"std"` - Enables use of the `std` library, primarily for floating point math. Also enables `"alloc"`.
* `"alloc"` - Enables the use of types that may allocate memory, such as [`Vec`][alloc::vec::Vec].

These features are disabled by default:

* `"libm"` - Uses the [`libm`](https://crates.io/crates/libm) floating point math library (for when the `std` feature is disabled).
* `"wide"` - Enables support for using SIMD types from [`wide`](https://crates.io/crates/wide).

### Embedded And `#![no_std]`

`palette_math` supports `#![no_std]` environments by disabling the `"std"` feature. It uses [`libm`], via the `"libm"` feature, to provide the floating-point operations that are typically in `std`, and the `"alloc"` feature to provide features that use allocating types.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
