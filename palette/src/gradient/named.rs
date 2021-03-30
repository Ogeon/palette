//! A collection of named gradient constants. Can be toggled with the `"named_gradients"`
//! Cargo feature.
//!
//! They are taken from the [new matplotlib gradients](https://github.com/BIDS/colormap/blob/master/colormaps.py).
//!
//! ```
//! use palette::gradient::named as grad_const;
//!
//! let pal = grad_const::MAGMA.take(5);

include!(concat!(env!("OUT_DIR"), "/named_gradients.rs"));
