use num::Float;

use std::ops::{Add, Sub};
use std::marker::PhantomData;

use {Alpha, Xyz, Lab, LabHue, Hue};
use {Limited, Mix, Shade, GetHue, FromColor, IntoColor, Saturate};
use {clamp, flt};
use white_point::{WhitePoint, D65};

///CIE L*C*h° with an alpha component. See the [`Lcha` implementation in `Alpha`](struct.Alpha.html#Lcha).
pub type Lcha<Wp, T = f32> = Alpha<Lch<Wp, T>, T>;



///CIE L*C*h°, a polar version of [CIE L*a*b*](struct.Lab.html).
///
///L*C*h° shares its range and perceptual uniformity with L*a*b*, but it's a
///cylindrical color space, like [HSL](struct.Hsl.html) and
///[HSV](struct.Hsv.html). This gives it the same ability to directly change
///the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, PartialEq)]
pub struct Lch<Wp = D65, T: Float = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///L* is the lightness of the color. 0.0 gives absolute black and 100.0
    ///give the brightest white.
    pub l: T,

    ///C* is the colorfulness of the color. It's similar to saturation. 0.0
    ///gives gray scale colors, and numbers around 128-181 gives fully
    ///saturated colors. The upper limit of 181 should include the whole
    ///L*a*b* space and some more.
    pub chroma: T,

    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: LabHue<T>,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Lch<Wp, T> { *self }
}

impl<T> Lch<D65, T>
    where T: Float,
{
    ///CIE L*C*h° with white point D65.
    pub fn new(l: T, chroma: T, hue: LabHue<T>) -> Lch<D65, T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE L*C*h°.
    pub fn with_wp(l: T, chroma: T, hue: LabHue<T>) -> Lch<Wp, T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            white_point: PhantomData,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<T> Alpha<Lch<D65, T>, T>
    where T: Float,
{
    ///CIE L*C*h° and transparency with white point D65.
    pub fn new(l: T, chroma: T, hue: LabHue<T>, alpha: T) -> Lcha<D65, T> {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha: alpha,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<Wp, T> Alpha<Lch<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE L*C*h° and transparency.
    pub fn with_wp(l: T, chroma: T, hue: LabHue<T>, alpha: T) -> Lcha<Wp, T> {
        Alpha {
            color: Lch::with_wp(l, chroma, hue),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let lab: Lab<Wp, T> = xyz.into_lab();
        Self::from_lab(lab)
    }

    fn from_lab(lab: Lab<Wp, T>) -> Self {
        Lch {
            l: lab.l,
            chroma: (lab.a * lab.a + lab.b * lab.b).sqrt(),
            hue: lab.get_hue().unwrap_or(LabHue::from(T::zero())),
            white_point: PhantomData,
        }
    }

    fn from_lch(lch: Lch<Wp, T>) -> Self {
        lch
    }

}

impl<Wp, T> Limited for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() &&
        self.chroma >= T::zero()
    }

    fn clamp(&self) -> Lch<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.chroma = self.chroma.max(T::zero())
    }
}

impl<Wp, T> Mix for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Lch<Wp, T>, factor: T) -> Lch<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Lch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: self.l + amount * flt(100.0),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn with_hue(&self, hue: LabHue<T>) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue,
            white_point: PhantomData,
        }
    }

    fn shift_hue(&self, amount: LabHue<T>) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma * (T::one() + factor),
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Lch<Wp, T> {
        Lch::with_wp(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl<Wp, T> Add<Lch<Wp, T>> for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lch<Wp, T>;

    fn add(self, other: Lch<Wp, T>) -> Lch<Wp, T> {
        Lch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lch<Wp, T>;

    fn add(self, c: T) -> Lch<Wp, T> {
        Lch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Lch<Wp, T>> for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lch<Wp, T>;

    fn sub(self, other: Lch<Wp, T>) -> Lch<Wp, T> {
        Lch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Lch<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lch<Wp, T>;

    fn sub(self, c: T) -> Lch<Wp, T> {
        Lch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use Lch;

    #[test]
    fn ranges() {
        assert_ranges!{
            Lch;
            limited {
                l: 0.0 => 100.0
            }
            limited_min {
                chroma: 0.0 => 200.0
            }
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
