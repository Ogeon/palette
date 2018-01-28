use num_traits::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Xyz, Yxy};
use {Limited, Mix, Shade, FromColor, Blend, ComponentWise};
use white_point::{WhitePoint, D65};
use {clamp, flt};
use blend::PreAlpha;

///Linear luminance with an alpha component. See the [`Lumaa` implementation in `Alpha`](struct.Alpha.html#Lumaa).
pub type Lumaa<Wp = D65, T = f32> = Alpha<Luma<Wp, T>, T>;

///Linear luminance.
///
///Luma is a purely gray scale color space, which is included more for
///completeness than anything else, and represents how bright a color is
///perceived to be. It's basically the `Y` component of [CIE
///XYZ](struct.Xyz.html). The lack of any form of hue representation limits
///the set of operations that can be performed on it.
#[derive(Debug, PartialEq)]
pub struct Luma<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,

}

impl<Wp, T> Copy for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Luma<Wp, T> { *self }
}

impl<T> Luma<D65, T>
    where T: Float,
{
    ///Linear luminance with white point D65.
    pub fn new(luma: T) -> Luma<D65, T> {
        Luma {
            luma: luma,
            white_point: PhantomData,
        }
    }

    ///Linear luminance from an 8 bit value with white point D65.
    pub fn new_u8(luma: u8) -> Luma<D65, T> {
        Luma {
            luma: flt::<T,_>(luma) / flt(255.0),
            white_point: PhantomData,
        }
    }

}

impl<Wp, T> Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear luminance.
    pub fn with_wp(luma: T) -> Luma<Wp, T> {
        Luma {
            luma: luma,
            white_point: PhantomData,
        }
    }

    ///Linear luminance from an 8 bit value.
    pub fn with_wp_u8(luma: u8) -> Luma<Wp, T> {
        Luma {
            luma: flt::<T,_>(luma) / flt(255.0),
            white_point: PhantomData,
        }
    }
}

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<T> Alpha<Luma<D65, T>, T>
    where T: Float,
{
    ///Linear luminance with transparency and white point D65.
    pub fn new(luma: T, alpha: T) -> Lumaa<D65, T> {
        Alpha {
            color: Luma::new(luma),
            alpha: alpha,
        }
    }

    ///Linear luminance and transparency from 8 bit values and white point D65.
    pub fn new_u8(luma: u8, alpha: u8) -> Lumaa<D65, T> {
        Alpha {
            color: Luma::new_u8(luma),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }
}

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<Wp, T> Alpha<Luma<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear luminance with transparency.
    pub fn with_wp(luma: T, alpha: T) -> Lumaa<Wp, T> {
        Alpha {
            color: Luma::with_wp(luma),
            alpha: alpha,
        }
    }

    ///Linear luminance and transparency from 8 bit values.
    pub fn with_wp_u8(luma: u8, alpha: u8) -> Lumaa<Wp, T> {
        Alpha {
            color: Luma::with_wp_u8(luma),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        Luma {
            luma: xyz.y,
            white_point: PhantomData,
        }
    }

    fn from_yxy(yxy: Yxy<Wp, T>) -> Self {
        Luma {
            luma: yxy.luma,
            white_point: PhantomData,
        }
    }

    fn from_luma(luma: Luma<Wp, T>) -> Self {
        luma
    }

}

impl<Wp, T> Limited for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.luma >= T::zero() && self.luma <= T::one()
    }

    fn clamp(&self) -> Luma<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.luma = clamp(self.luma, T::zero(), T::one());
    }
}

impl<Wp, T> Mix for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Luma<Wp, T>, factor: T) -> Luma<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Luma {
            luma: self.luma + factor * (other.luma - self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Luma<Wp, T> {
        Luma {
            luma: (self.luma + amount).max(T::zero()),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Blend for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Color = Luma<Wp, T>;

    fn into_premultiplied(self) -> PreAlpha<Luma<Wp, T>, T> {
        Lumaa::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Luma<Wp, T>, T>) -> Self {
        Lumaa::from(color).into()
    }
}

impl<Wp, T> ComponentWise for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Luma<Wp, T>, mut f: F) -> Luma<Wp, T> {
        Luma {
            luma: f(self.luma, other.luma),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Luma<Wp, T> {
        Luma {
            luma: f(self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Luma<Wp, T> {
        Luma::with_wp(T::zero())
    }
}

impl<Wp, T> Add<Luma<Wp, T>> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn add(self, other: Luma<Wp, T>) -> Luma<Wp, T> {
        Luma {
            luma: self.luma + other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn add(self, c: T) -> Luma<Wp, T> {
        Luma {
            luma: self.luma + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Luma<Wp, T>> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn sub(self, other: Luma<Wp, T>) -> Luma<Wp, T> {
        Luma {
            luma: self.luma - other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn sub(self, c: T) -> Luma<Wp, T> {
        Luma {
            luma: self.luma - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<Luma<Wp, T>> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn mul(self, other: Luma<Wp, T>) -> Luma<Wp, T> {
        Luma {
            luma: self.luma * other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn mul(self, c: T) -> Luma<Wp, T> {
        Luma {
            luma: self.luma * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<Luma<Wp, T>> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn div(self, other: Luma<Wp, T>) -> Luma<Wp, T> {
        Luma {
            luma: self.luma / other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Luma<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Luma<Wp, T>;

    fn div(self, c: T) -> Luma<Wp, T> {
        Luma {
            luma: self.luma / c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use Luma;

    #[test]
    fn ranges() {
        assert_ranges!{
            Luma;
            limited {
                luma: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
