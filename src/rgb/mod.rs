//!RGB types, spaces and standards.

use num_traits::Float;
use approx::ApproxEq;

use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;

use {Alpha, Hsl, Hsv, Luma, RgbHue, Xyz, Yxy};
use {Blend, ComponentWise, FromColor, GetHue, Limited, Mix, Shade};
use white_point::{D65, WhitePoint};
use matrix::{matrix_inverse, multiply_xyz_to_rgb, rgb_to_xyz_matrix};
use {clamp, flt};
use pixel::{GammaRgb, RgbPixel, Srgb, TransferFn};
use blend::PreAlpha;

pub mod standards;

///An RGB space and a transfer function.
pub trait RgbStandard {
    ///The RGB color space.
    type Space: RgbSpace;

    ///The transfer function for the color components.
    type TransferFn: TransferFn;
}

impl<S: RgbSpace, T: TransferFn> RgbStandard for (S, T) {
    type Space = S;
    type TransferFn = T;
}

impl<P: Primaries, W: WhitePoint, T: TransferFn> RgbStandard for (P, W, T) {
    type Space = (P, W);
    type TransferFn = T;
}

///A set of primaries and a white point.
pub trait RgbSpace {
    ///The primaries of the RGB color space.
    type Primaries: Primaries;

    ///The white point of the RGB color space.
    type WhitePoint: WhitePoint;
}

impl<P: Primaries, W: WhitePoint> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

///Represents the red, green and blue primaries of an RGB space.
pub trait Primaries {
    ///Primary red.
    fn red<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
    ///Primary green.
    fn green<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
    ///Primary blue.
    fn blue<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
}

///Linear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](struct.Alpha.html#Rgba).
pub type Rgba<S = self::standards::Srgb, T = f32> = Alpha<Rgb<S, T>, T>;

///Linear RGB.
///
///RGB is probably the most common color space, when it comes to computer
///graphics, and it's defined as an additive mixture of red, green and blue
///light, where gray scale colors are created when these three channels are
///equal in strength. This particular RGB type is based on the ITU-R BT.709
///primaries, which makes it a linear version of sRGB.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from a
///displayable RGB, such as sRGB. See the [`pixel`](pixel/index.html) module
///for encoding types.
#[derive(Debug, PartialEq)]
pub struct Rgb<S = self::standards::Srgb, T = f32>
where
    T: Float,
    S: RgbSpace,
{
    ///The amount of red light, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where 0.0 is no green light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where 0.0 is no blue light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
}

impl<S, T> Clone for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn clone(&self) -> Rgb<S, T> {
        *self
    }
}

impl<T> Rgb<self::standards::Srgb, T>
where
    T: Float,
{
    ///Linear RGB with white point D65.
    pub fn new(red: T, green: T, blue: T) -> Rgb<self::standards::Srgb, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            space: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values with whtie point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Rgb<self::standards::Srgb, T> {
        Rgb {
            red: flt::<T, _>(red) / flt(255.0),
            green: flt::<T, _>(green) / flt(255.0),
            blue: flt::<T, _>(blue) / flt(255.0),
            space: PhantomData,
        }
    }
}

impl<S, T> Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    ///Linear RGB.
    pub fn with_wp(red: T, green: T, blue: T) -> Rgb<S, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            space: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8) -> Rgb<S, T> {
        Rgb {
            red: flt::<T, _>(red) / flt(255.0),
            green: flt::<T, _>(green) / flt(255.0),
            blue: flt::<T, _>(blue) / flt(255.0),
            space: PhantomData,
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgb<S, T> {
        let (r, g, b, _) = pixel.to_rgba();
        Rgb::with_wp(r, g, b)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::new(0.5, 0.3, 0.1);
    ///assert_eq!((c.red, c.green, c.blue), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            T::one(),
        )
    }
}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<T> Alpha<Rgb<self::standards::Srgb, T>, T>
where
    T: Float,
{
    ///Linear RGB with transparency and with white point D65.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Rgba<self::standards::Srgb, T> {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values and with white point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<self::standards::Srgb, T> {
        Alpha {
            color: Rgb::new_u8(red, green, blue),
            alpha: flt::<T, _>(alpha) / flt(255.0),
        }
    }
}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<S, T> Alpha<Rgb<S, T>, T>
where
    T: Float,
    S: RgbSpace,
{
    ///Linear RGB with transparency.
    pub fn with_wp(red: T, green: T, blue: T, alpha: T) -> Rgba<S, T> {
        Alpha {
            color: Rgb::with_wp(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<S, T> {
        Alpha {
            color: Rgb::with_wp_u8(red, green, blue),
            alpha: flt::<T, _>(alpha) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgba<S, T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgba::with_wp(r, g, b, a)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgba;
    ///
    ///let c = Rgba::new(0.5, 0.3, 0.1, 0.5);
    ///assert_eq!((c.red, c.green, c.blue, c.alpha), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1, 0.5), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one()),
        )
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace<WhitePoint = Wp>,
    Wp: WhitePoint,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let transform_matrix = matrix_inverse(&rgb_to_xyz_matrix::<S, T>());
        multiply_xyz_to_rgb(&transform_matrix, &xyz)
    }

    fn from_rgb<Sp: RgbSpace<WhitePoint = Wp>>(rgb: Rgb<Sp, T>) -> Self {
        Self::from_xyz(Xyz::from_rgb(rgb))
    }

    fn from_hsl(hsl: Hsl<Wp, T>) -> Self {
        let c = (T::one() - (hsl.lightness * flt(2.0) - T::one()).abs()) * hsl.saturation;
        let h = hsl.hue.to_positive_degrees() / flt(60.0);
        let x = c * (T::one() - (h % flt(2.0) - T::one()).abs());
        let m = hsl.lightness - c * flt(0.5);

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < flt(2.0) {
            (x, c, T::zero())
        } else if h >= flt(2.0) && h < flt(3.0) {
            (T::zero(), c, x)
        } else if h >= flt(3.0) && h < flt(4.0) {
            (T::zero(), x, c)
        } else if h >= flt(4.0) && h < flt(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            space: PhantomData,
        }
    }

    fn from_hsv(hsv: Hsv<Wp, T>) -> Self {
        let c = hsv.value * hsv.saturation;
        let h = hsv.hue.to_positive_degrees() / flt(60.0);
        let x = c * (T::one() - (h % flt(2.0) - T::one()).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < flt(2.0) {
            (x, c, T::zero())
        } else if h >= flt(2.0) && h < flt(3.0) {
            (T::zero(), c, x)
        } else if h >= flt(3.0) && h < flt(4.0) {
            (T::zero(), x, c)
        } else if h >= flt(4.0) && h < flt(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            space: PhantomData,
        }
    }

    fn from_luma(luma: Luma<Wp, T>) -> Self {
        Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            space: PhantomData,
        }
    }
}

impl<S, T> Limited for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() && self.green >= T::zero()
            && self.green <= T::one() && self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> Rgb<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, T::zero(), T::one());
        self.green = clamp(self.green, T::zero(), T::one());
        self.blue = clamp(self.blue, T::zero(), T::one());
    }
}

impl<S, T> Mix for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn mix(&self, other: &Rgb<S, T>, factor: T) -> Rgb<S, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            space: PhantomData,
        }
    }
}

impl<S, T> Shade for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Rgb<S, T> {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            space: PhantomData,
        }
    }
}

impl<S, T> GetHue for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = flt(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians(
                (sqrt_3 * (self.green - self.blue))
                    .atan2(self.red * flt(2.0) - self.green - self.blue),
            ))
        }
    }
}

impl<S, T> Blend for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Color = Rgb<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Rgb<S, T>, T> {
        Rgba::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Rgb<S, T>, T>) -> Self {
        Rgba::from(color).into()
    }
}

impl<S, T> ComponentWise for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Rgb<S, T>, mut f: F) -> Rgb<S, T> {
        Rgb {
            red: f(self.red, other.red),
            green: f(self.green, other.green),
            blue: f(self.blue, other.blue),
            space: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Rgb<S, T> {
        Rgb {
            red: f(self.red),
            green: f(self.green),
            blue: f(self.blue),
            space: PhantomData,
        }
    }
}

impl<S, T> Default for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn default() -> Rgb<S, T> {
        Rgb::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<S, T> Add<Rgb<S, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn add(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            space: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn add(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<Rgb<S, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn sub(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn sub(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            space: PhantomData,
        }
    }
}

impl<S, T> Mul<Rgb<S, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn mul(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            space: PhantomData,
        }
    }
}

impl<S, T> Mul<T> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn mul(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            space: PhantomData,
        }
    }
}

impl<S, T> Div<Rgb<S, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn div(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            space: PhantomData,
        }
    }
}

impl<S, T> Div<T> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Rgb<S, T>;

    fn div(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            space: PhantomData,
        }
    }
}

impl<S, T> From<Alpha<Rgb<S, T>, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn from(color: Alpha<Rgb<S, T>, T>) -> Rgb<S, T> {
        color.color
    }
}

impl<T> From<Srgb<D65, T>> for Rgb<::rgb::standards::Srgb, T>
where
    T: Float,
{
    fn from(srgb: Srgb<D65, T>) -> Rgb<::rgb::standards::Srgb, T> {
        srgb.to_linear().into()
    }
}

impl<S, T> From<GammaRgb<S::WhitePoint, T>> for Rgb<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn from(gamma_rgb: GammaRgb<S::WhitePoint, T>) -> Rgb<S, T> {
        gamma_rgb.to_linear().into()
    }
}

impl<T> From<Srgb<D65, T>> for Alpha<Rgb<::rgb::standards::Srgb, T>, T>
where
    T: Float,
{
    fn from(srgb: Srgb<D65, T>) -> Alpha<Rgb<::rgb::standards::Srgb, T>, T> {
        srgb.to_linear()
    }
}

impl<S, T> From<GammaRgb<S::WhitePoint, T>> for Alpha<Rgb<S, T>, T>
where
    T: Float,
    S: RgbSpace,
{
    fn from(gamma_rgb: GammaRgb<S::WhitePoint, T>) -> Alpha<Rgb<S, T>, T> {
        gamma_rgb.to_linear()
    }
}

impl<S, T> ApproxEq for Rgb<S, T>
where
    T: Float + ApproxEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace,
{
    type Epsilon = <T as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.red.relative_eq(&other.red, epsilon, max_relative)
            && self.green.relative_eq(&other.green, epsilon, max_relative)
            && self.blue.relative_eq(&other.blue, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.red.ulps_eq(&other.red, epsilon, max_ulps)
            && self.green.ulps_eq(&other.green, epsilon, max_ulps)
            && self.blue.ulps_eq(&other.blue, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use Rgb;
    use rgb::standards::Srgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Rgb<Srgb, f64>;
            limited {
                red: 0.0 => 1.0,
                green: 0.0 => 1.0,
                blue: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
