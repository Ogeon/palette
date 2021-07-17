//! Linear encoding

use core::marker::PhantomData;

use crate::encoding::TransferFn;
use crate::luma::LumaStandard;
use crate::rgb::{RgbSpace, RgbStandard};
use crate::white_point::WhitePoint;

/// A generic standard with linear components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Linear<S>(PhantomData<S>);

impl<T, Sp> RgbStandard<T> for Linear<Sp>
where
    Sp: RgbSpace<T>,
{
    type Space = Sp;
    type TransferFn = LinearFn;
}

impl<T, Wp> LumaStandard<T> for Linear<Wp>
where
    Wp: WhitePoint<T>,
{
    type WhitePoint = Wp;
    type TransferFn = LinearFn;
}

/// Linear color component encoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LinearFn;

impl<T> TransferFn<T> for LinearFn {
    #[inline(always)]
    fn into_linear(x: T) -> T {
        x
    }

    #[inline(always)]
    fn from_linear(x: T) -> T {
        x
    }
}
