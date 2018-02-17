//! Linear RGB

use std::marker::PhantomData;

use num_traits::Float;
use rgb::{RgbSpace, RgbStandard};
use rgb::standards::Srgb;
use pixel::TransferFn;

/// A generic RGB standard with linear components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Linear<S: RgbSpace = Srgb>(PhantomData<S>);

impl<S: RgbSpace> RgbStandard for Linear<S> {
    type Space = S;
    type TransferFn = LinearFn;
}

///Linear color component encoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LinearFn;

impl TransferFn for LinearFn {
    #[inline(always)]
    fn into_linear<T: Float>(x: T) -> T {
        x
    }

    #[inline(always)]
    fn from_linear<T: Float>(x: T) -> T {
        x
    }
}
