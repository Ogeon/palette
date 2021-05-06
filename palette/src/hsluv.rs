use core::marker::PhantomData;

use approx::{AbsDiffEq, RelativeEq};
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::{ColorDifference, Lab, luv_bounds::LuvBounds};
use crate::{
    convert::{FromColor, FromColorUnclamped}, white_point::WhitePoint, Lch,
};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, FloatComponent, GetHue,
    Hue, LabHue, Mix, Pixel, RelativeContrast, Saturate, Shade, Xyz,
};
use crate::{encoding::pixel::RawPixel, white_point::D65};

/// `HSLuv` with alpha an alpha component. See the [`Hsluva`
/// implementation in `Alpha`](crate::Alpha#Hsluv).
pub type Hsluva<Wp, T> = Alpha<Hsluv<Wp, T>, T>;

/// The HSLuv color space.
///
/// [HSLuv](https://www.hsluv.org/) is designed as a "human-friendly
/// alternative to HSL". It is an extension of the CIELuv / CIELch
/// format, where the chroma component is dynamically scaled based on
/// lightness to effectively create a saturation component similar to
/// HSL.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Lch, Hsluv)
)]
#[repr(C)]
pub struct Hsluv<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LabHue<T>,

    /// The colorfulness of the color. 0.0 gives gray scale colors and 100.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how light the color will look. 0.0 will be black, 0.5 will give
    /// a clear color, and 100.0 will give white.
    pub l: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}
impl<Wp, T> Copy for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Hsluv<Wp, T> {
        *self
    }
}

impl<T> Hsluv<D65, T>
where
    T: FloatComponent,
{
    /// HSLuv with default D65 white point.
    pub fn new<H: Into<LabHue<T>>>(hue: H, saturation: T, l: T) -> Self {
        Hsluv {
            hue: hue.into(),
            saturation,
            l,
            white_point: PhantomData,
        }
    }
}
impl<Wp, T> Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// HSLuv with sepcified D65 white point.
    pub fn with_wp<H: Into<LabHue<T>>>(hue: H, saturation: T, l: T) -> Self {
        Hsluv {
            hue: hue.into(),
            saturation,
            l,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(hue, saturation, L\*)` tuple.
    pub fn into_components(self) -> (LabHue<T>, T, T) {
        (self.hue, self.saturation, self.l)
    }

    /// Convert from a `(hue, saturation, L\*)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((hue, saturation, l): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, l)
    }

    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(100.0)
    }

    /// Return the minimum `saturation` value.
    pub fn min_saturation() -> T {
        T::zero()
    }
    /// Return the maximum `saturation` value.
    pub fn max_saturation() -> T {
        from_f64(100.0)
    }

    /// Return sum of the absolute difference of components.
    fn abs_diff(&self, other: &Self) -> T {
	(self.hue - other.hue).to_positive_degrees() + (self.saturation - other.saturation).abs() +
	    (self.l - other.l).abs()
    }

    fn max_component(&self) -> T {
	let h = self.hue.to_positive_degrees();
	if h > self.l {
	    if h > self.saturation {
		h
	    } else {
		self.saturation
	    }
	} else if self.l > self.saturation {
	    self.l
	} else {
	    self.saturation
	}

    }

    /// Return the maximum `chroma` value that is representable for
    /// the given lightness and hue values.
    pub fn max_chroma_for_lh(&self) -> T {
        LuvBounds::from_lightness(self.l).max_chroma_at_hue(self.hue)
    }
}

impl<Wp, T> PartialEq for Hsluv<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue && self.saturation == other.saturation && self.l == other.l
    }
}

impl<Wp, T> Eq for Hsluv<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Hsluva"></span>[`Hsluva`](crate::Hsluva) implementations.
impl<T, A> Alpha<Hsluv<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE L\*C\*h° and transparency with white point D65.
    pub fn new<H: Into<LabHue<T>>>(hue: H, saturation: T, l: T, alpha: A) -> Self {
        Alpha {
            color: Hsluv::new(hue, saturation, l),
            alpha,
        }
    }
}

///<span id="Hsluva"></span>[`Hsluva`](crate::Hsluva) implementations.
impl<Wp, T, A> Alpha<Hsluv<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE L\*C\*h° and transparency.
    pub fn with_wp<H: Into<LabHue<T>>>(hue: H, saturation: T, l: T, alpha: A) -> Self {
        Alpha {
            color: Hsluv::with_wp(hue, saturation, l),
            alpha,
        }
    }

    /// Convert to a `(h°, saturation, L\*, alpha)` tuple.
    pub fn into_components(self) -> (LabHue<T>, T, T, A) {
        (self.hue, self.saturation, self.l, self.alpha)
    }

    /// Convert from a `(h°, saturation, L\*, alpha)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((hue, saturation, l, alpha): (H, T, T, A)) -> Self {
        Self::with_wp(hue, saturation, l, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        if color.l > T::from_f64(99.999999) {
            Hsluv::with_wp(color.hue, T::zero(), T::from_f64(100.0))
        } else if color.l < T::from_f64(1e-6) {
            Hsluv::with_wp(color.hue, T::zero(), T::zero())
        } else {
            let max_chroma = color.max_chroma_for_lh();
            Hsluv::with_wp(color.hue, color.chroma / max_chroma * T::from_f64(100.0), color.l)
        }
    }
}

impl<Wp, T> FromColorUnclamped<Hsluv<Wp, T>> for Hsluv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Hsluv<Wp, T>) -> Self {
        color
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LabHue<T>>> From<(H, T, T)> for Hsluv<Wp, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> From<Hsluv<Wp, T>> for (LabHue<T>, T, T) {
    fn from(val: Hsluv<Wp, T>) -> Self {
        val.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LabHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsluv<Wp, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> From<Alpha<Hsluv<Wp, T>, A>>
    for (LabHue<T>, T, T, A)
{
    fn from(val: Alpha<Hsluv<Wp, T>, A>) -> Self {
        val.into_components()
    }
}


impl<Wp, T> Clamp for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn is_within_bounds(&self) -> bool {
        self.l >= Self::min_l() && self.l <= Self::max_l() &&
	    self.saturation >= Self::min_saturation() &&
	    self.saturation <= Self::max_saturation()
    }

    fn clamp(&self) -> Hsluv<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, Self::min_l(), Self::max_l());
        self.saturation = clamp(self.saturation, Self::min_saturation(), Self::max_saturation());
    }
}

/// NOTE: This seems like a reasonable implementation, but I'm not
/// saturation is a precentage of a quantity (chroma) that changes
/// with lightness. an alternative would be to convert to Lch and mix
/// there.
impl<Wp, T> Mix for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Hsluv<Wp, T>, factor: T) -> Hsluv<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Hsluv {
            l: self.l + factor * (other.l - self.l),
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            hue: self.hue + factor * hue_diff,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Hsluv<Wp, T> {
        let difference = if factor >= T::zero() {
            T::from_f64(100.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Hsluv {
            l: (self.l + delta).max(T::zero()),
	    ..*self
        }
    }

    fn lighten_fixed(&self, amount: T) -> Hsluv<Wp, T> {
        Hsluv {
            l: (self.l + T::from_f64(100.0) * amount).max(T::zero()),
	    ..*self
        }
    }
}

impl<Wp, T> GetHue for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsluv<Wp, T> {
        Hsluv {
            hue: hue.into(),
	    ..*self
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsluv<Wp, T> {
        Hsluv {
            hue: self.hue + amount.into(),
	    ..*self
        }
    }
}

/// CIEDE2000 distance metric for color difference.
impl<Wp, T> ColorDifference for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn get_color_difference(&self, other: &Hsluv<Wp, T>) -> Self::Scalar {
	let lab1: Lab<Wp, T> = Lab::from_color(*self);
	let lab2: Lab<Wp, T> = Lab::from_color(*other);

	lab1.get_color_difference(&lab2)
    }
}

impl<Wp, T> Saturate for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsluv<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_saturation() - self.saturation
        } else {
            self.saturation
        };

        let delta = difference.max(T::zero()) * factor;

        Hsluv {
            saturation: clamp(self.saturation + delta, Self::min_saturation(), Self::max_saturation()),
	    ..*self
        }
    }

    fn saturate_fixed(&self, amount: T) -> Hsluv<Wp, T> {
        Hsluv {
            saturation: clamp(self.saturation + Self::max_saturation() * amount,
			      Self::min_saturation(),
			      Self::max_saturation()),
	    ..*self
        }
    }
}

impl<Wp, T> Default for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Hsluv<Wp, T> {
        Hsluv::with_wp(LabHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<Wp, T, P> AsRef<P> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Hsluv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl<Wp, T> AbsDiffEq for Hsluv<Wp, T>
where Wp: WhitePoint,
      T: FloatComponent,
{
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon {
	T::from_f64(1.0e-5)
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
	self.abs_diff(other) < epsilon
    }
}

impl<Wp, T> RelativeEq for Hsluv<Wp, T>
where Wp: WhitePoint,
      T: FloatComponent,
{
    fn default_max_relative() -> Self::Epsilon {
	T::from_f64(1.0e-6)
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
	let abs_diff = self.abs_diff(other);
	if abs_diff <  epsilon {
	    return true;
	}
	let max_self = self.max_component();
	let other_self = other.max_component();

	let largest = if max_self > other_self {
	    max_self
	} else {
	    other_self
	};

	abs_diff <= largest * max_relative
    }
}

#[cfg(feature = "random")]
impl<Wp, T> Distribution<Hsluv<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsluv<Wp, T> {
        Hsluv {
            hue: rng.gen::<LabHue<T>>(),
            saturation: crate::Float::sqrt(rng.gen()) * from_f64(100.0),
            l: rng.gen() * from_f64(100.0),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformHsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    hue: crate::hues::UniformLabHue<T>,
    saturation: Uniform<T>,
    l: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Hsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformHsluv<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformHsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Hsluv<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformHsluv {
            hue: crate::hues::UniformLabHue::new(low.hue, high.hue),
            saturation: Uniform::new::<_, T>(low.saturation * low.saturation, high.saturation * high.saturation),
            l: Uniform::new::<_, T>(low.l, high.l),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformHsluv {
            hue: crate::hues::UniformLabHue::new_inclusive(low.hue, high.hue),
            saturation: Uniform::new_inclusive::<_, T>(
                low.saturation * low.saturation,
                high.saturation * high.saturation,
            ),
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsluv<Wp, T> {
        Hsluv {
            hue: self.hue.sample(rng),
            saturation: crate::Float::sqrt(self.saturation.sample(rng)),
            l: self.l.sample(rng),
            white_point: PhantomData,
        }
    }
}



#[cfg(test)]
mod test {
    use crate::white_point::D65;
    use crate::Hsluv;

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsluv<D65, f64>;
            clamped {
                saturation: 0.0 => 100.0,
                l: 0.0 => 100.0
            }
	    clamped_min {
	    }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsluv<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Hsluv<D65>: l, chroma, hue);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Hsluv::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Hsluv::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Hsluv::<D65, f32>::min_saturation(), 0.0);
        assert_relative_eq!(Hsluv::<D65, f32>::max_saturation(), 100.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsluv::new(120.0, 30.0, 60.0)).unwrap();

        assert_eq!(serialized, r#"{"hue":120.0,"saturation":30.0,"l":60.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsluv =
            ::serde_json::from_str(r#"{"hue":120.0,"saturation":30.0,"l":60.0}"#).unwrap();

        assert_eq!(deserialized, Hsluv::new(120.0, 30.0, 60.0));
    }

    // #[cfg(feature = "random")]
    // test_uniform_distribution! {
    //     Hsluv<D65, f32> as crate::Lab {
    //         l: (0.0, 100.0),
    //         a: (-89.0, 89.0),
    //         b: (-89.0, 89.0),
    //     },
    //     min: Hsluv::new(0.0f32, 0.0, 0.0),
    //     max: Hsluv::new(360.0, 100.0, 100.0)
    // }

}
