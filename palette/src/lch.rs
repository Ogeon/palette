use float::Float;

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use encoding::pixel::RawPixel;
use white_point::{D65, WhitePoint};
use {cast, clamp};
use {Alpha, Hue, Lab, LabHue, Xyz};
use {Component, FromColor, GetHue, IntoColor, Limited, Mix, Pixel, Saturate, Shade};

/// CIE L\*C\*h° with an alpha component. See the [`Lcha` implementation in
/// `Alpha`](struct.Alpha.html#Lcha).
pub type Lcha<Wp, T = f32> = Alpha<Lch<Wp, T>, T>;

///CIE L\*C\*h°, a polar version of [CIE L\*a\*b\*](struct.Lab.html).
///
///L\*C\*h° shares its range and perceptual uniformity with L\*a\*b\*, but
/// it's a cylindrical color space, like [HSL](struct.Hsl.html) and
///[HSV](struct.Hsv.html). This gives it the same ability to directly change
///the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_white_point = "Wp"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Lab, Lch)]
#[repr(C)]
pub struct Lch<Wp = D65, T = f32>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///L\* is the lightness of the color. 0.0 gives absolute black and 100.0
    ///gives the brightest white.
    pub l: T,

    ///C\* is the colorfulness of the color. It's similar to saturation. 0.0
    ///gives gray scale colors, and numbers around 128-181 gives fully
    ///saturated colors. The upper limit of 128 should
    ///include the whole L\*a\*b\* space and some more.
    pub chroma: T,

    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    #[palette_unsafe_same_layout_as = "T"]
    pub hue: LabHue<T>,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lch<Wp, T> {
        *self
    }
}

impl<T> Lch<D65, T>
where
    T: Component + Float,
{
    ///CIE L\*C\*h° with white point D65.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H) -> Lch<D65, T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///CIE L\*C\*h°.
    pub fn with_wp<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H) -> Lch<Wp, T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*, h°)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*, h°)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::with_wp(l, chroma, hue)
    }
}

///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<T, A> Alpha<Lch<D65, T>, A>
where
    T: Component + Float,
    A: Component,
{
    ///CIE L\*C\*h° and transparency with white point D65.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha: alpha,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<Wp, T, A> Alpha<Lch<Wp, T>, A>
where
    T: Component + Float,
    A: Component,
    Wp: WhitePoint,
{
    ///CIE L\*C\*h° and transparency.
    pub fn with_wp<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::with_wp(l, chroma, hue),
            alpha: alpha,
        }
    }

    /// Convert to a `(L\*, C\*, h°, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>, A) {
        (self.l, self.chroma, self.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*, h°, alpha)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::with_wp(l, chroma, hue, alpha)
    }
}

impl<Wp, T> From<Xyz<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Xyz<Wp, T>) -> Self {
        let lab: Lab<Wp, T> = color.into_lab();
        Self::from_lab(lab)
    }
}

impl<Wp, T> From<Lab<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Lab<Wp, T>) -> Self {
        Lch {
            l: color.l,
            chroma: (color.a * color.a + color.b * color.b).sqrt(),
            hue: color.get_hue().unwrap_or(LabHue::from(T::zero())),
            white_point: PhantomData,
        }
    }
}

impl<Wp: WhitePoint, T: Component + Float, H: Into<LabHue<T>>> From<(T, T, H)> for Lch<Wp, T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: Component + Float> Into<(T, T, LabHue<T>)> for Lch<Wp, T> {
    fn into(self) -> (T, T, LabHue<T>) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: Component + Float, H: Into<LabHue<T>>, A: Component> From<(T, T, H, A)>
    for Alpha<Lch<Wp, T>, A>
{
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: Component + Float, A: Component> Into<(T, T, LabHue<T>, A)>
    for Alpha<Lch<Wp, T>, A>
{
    fn into(self) -> (T, T, LabHue<T>, A) {
        self.into_components()
    }
}

impl<Wp, T> Limited for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= cast(100.0) && self.chroma >= T::zero()
    }

    fn clamp(&self) -> Lch<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), cast(100.0));
        self.chroma = self.chroma.max(T::zero())
    }
}

impl<Wp, T> Mix for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: self.l + amount * cast(100.0),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount.into(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn default() -> Lch<Wp, T> {
        Lch::with_wp(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl<Wp, T> Add<Lch<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn add(self, other: Lch<Wp, T>) -> Self::Output {
        Lch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn add(self, c: T) -> Self::Output {
        Lch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> AddAssign<Lch<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, other: Lch<Wp, T>) {
        self.l += other.l;
        self.chroma += other.chroma;
        self.hue += other.hue;
    }
}

impl<Wp, T> AddAssign<T> for Lch<Wp, T>
    where
        T: Component + Float + AddAssign,
        Wp: WhitePoint,
{
    fn add_assign(&mut self, c: T) {
        self.l += c;
        self.chroma += c;
        self.hue += c;
    }
}

impl<Wp, T> Sub<Lch<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn sub(self, other: Lch<Wp, T>) -> Self::Output {
        Lch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn sub(self, c: T) -> Self::Output {
        Lch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SubAssign<Lch<Wp, T>> for Lch<Wp, T>
where
    T: Component + Float + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, other: Lch<Wp, T>) {
        self.l -= other.l;
        self.chroma -= other.chroma;
        self.hue -= other.hue;
    }
}

impl<Wp, T> SubAssign<T> for Lch<Wp, T>
where
    T: Component + Float + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, c: T) {
        self.l -= c;
        self.chroma -= c;
        self.hue -= c;
    }
}

impl<Wp, T, P> AsRef<P> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lch<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

#[cfg(test)]
mod test {
    use white_point::D65;
    use Lch;

    #[test]
    fn ranges() {
        assert_ranges!{
            Lch<D65, f64>;
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

    raw_pixel_conversion_tests!(Lch<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Lch<D65>: l, chroma, hue);

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Lch::new(0.3, 0.8, 0.1));
    }
}
