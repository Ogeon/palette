use num::Float;

use std::ops::{Add, Sub};
use std::marker::PhantomData;

use { Alpha, Limited, Mix, Shade, GetHue, Hue, Rgb, Luma, Xyz, Yxy, Hsv, Hsl, Saturate, LabHue, WhitePoint, D65, clamp};
use lab::LabSpace;

pub type Lch<T> = LchSpace<D65, T>;
pub type Lcha<T> = AlphaLchSpace<D65, T>;

///CIE L*C*h° with an alpha component. See the [`Lcha` implementation in `Alpha`](struct.Alpha.html#Lcha).
pub type AlphaLchSpace<WP, T = f32> = Alpha<LchSpace<WP, T>, T>;

///CIE L*C*h°, a polar version of [CIE L*a*b*](struct.Lab.html).
///
///L*C*h° shares its range and perceptual uniformity with L*a*b*, but it's a
///cylindrical color space, like [HSL](struct.Hsl.html) and
///[HSV](struct.Hsv.html). This gives it the same ability to directly change
///the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, PartialEq)]
pub struct LchSpace<WP = D65, T = f32>
where T: Float,
    WP: WhitePoint<T>
{
    ///L* is the lightness of the color. 0.0 gives absolute black and 1.0
    ///give the brightest white.
    pub l: T,

    ///C* is the colorfulness of the color. It's similar to saturation. 0.0
    ///gives gray scale colors, and numbers around 1.0-1.41421356 gives fully
    ///saturated colors. The upper limit of 1.41421356 (or `sqrt(2.0)`) should
    ///include the whole L*a*b* space and some more.
    pub chroma: T,

    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: LabHue<T>,

    _wp: PhantomData<WP>,
}

impl<WP, T> Copy for LchSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{}

impl<WP, T> Clone for LchSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn clone(&self) -> LchSpace<WP,T> { *self }
}

impl<WP, T> LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    ///CIE L*C*h°.
    pub fn new(l: T, chroma: T, hue: LabHue<T>) -> LchSpace<WP, T> {
        LchSpace {
            l: l,
            chroma: chroma,
            hue: hue,
            _wp: PhantomData,
        }
    }
}


///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<WP, T> Alpha<LchSpace<WP, T>, T>
    where T: Float,
        WP: WhitePoint<T>
{
    ///CIE L*C*h° and transparency.
    pub fn new(l: T, chroma: T, hue: LabHue<T>, alpha: T) -> Lcha<T> {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha: alpha,
        }
    }
}

impl<WP, T> Limited for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() &&
        self.chroma >= T::zero()
    }

    fn clamp(&self) -> LchSpace<WP, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.chroma = self.chroma.max(T::zero())
    }
}

impl<WP, T> Mix for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &LchSpace<WP, T>, factor: T) -> LchSpace<WP, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        LchSpace {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Shade for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l + amount,
            chroma: self.chroma,
            hue: self.hue,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> GetHue for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
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

impl<WP, T> Hue for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn with_hue(&self, hue: LabHue<T>) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l,
            chroma: self.chroma,
            hue: hue,
            _wp: PhantomData,
        }
    }

    fn shift_hue(&self, amount: LabHue<T>) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Saturate for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l,
            chroma: self.chroma * (T::one() + factor),
            hue: self.hue,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Default for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn default() -> LchSpace<WP, T> {
        LchSpace::new(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl<WP, T> Add<LchSpace<WP, T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LchSpace<WP, T>;

    fn add(self, other: LchSpace<WP, T>) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Add<T> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LchSpace<WP, T>;

    fn add(self, c: T) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Sub<LchSpace<WP, T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LchSpace<WP, T>;

    fn sub(self, other: LchSpace<WP, T>) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Sub<T> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LchSpace<WP, T>;

    fn sub(self, c: T) -> LchSpace<WP, T> {
        LchSpace {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
            _wp: PhantomData,
        }
    }
}

// from_color!(to Lch from Rgb, Luma, Xyz, Yxy, Lab, Hsv, Hsl);

// alpha_from!(LchSpace {Rgb, Xyz, Yxy, Luma, Lab, Hsv, Hsl, Color});

impl<WP, T> From<LabSpace<WP, T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(lab: LabSpace<WP, T>) -> LchSpace<WP, T> {
        LchSpace {
            l: lab.l,
            chroma: (lab.a * lab.a + lab.b * lab.b).sqrt(),
            hue: lab.get_hue().unwrap_or(LabHue::from(T::zero())),
            _wp: PhantomData,
        }
    }
}

impl<WP, T> From<Rgb<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(rgb: Rgb<T>) -> LchSpace<WP, T> {
        LabSpace::from(rgb).into()
    }
}

impl<WP, T> From<Luma<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(luma: Luma<T>) -> LchSpace<WP, T> {
        LabSpace::from(luma).into()
    }
}

impl<WP, T> From<Xyz<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(xyz: Xyz<T>) -> LchSpace<WP, T> {
        LabSpace::from(xyz).into()
    }
}

impl<WP, T> From<Yxy<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(yxy: Yxy<T>) -> LchSpace<WP, T> {
        LabSpace::from(yxy).into()
    }
}

impl<WP, T> From<Hsv<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(hsv: Hsv<T>) -> LchSpace<WP, T> {
        LabSpace::from(hsv).into()
    }
}

impl<WP, T> From<Hsl<T>> for LchSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(hsl: Hsl<T>) -> LchSpace<WP, T> {
        LabSpace::from(hsl).into()
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
                l: 0.0 => 1.0
            }
            limited_min {
                chroma: 0.0 => 2.0
            }
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
