use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Luma, Xyz, Hsv, Hsl, RgbHue};
use {Limited, Mix, Shade, GetHue, FromColor, Blend, ComponentWise};
use white_point::{WhitePoint, D65};
use rgb_profile::{RgbPrimaries, SrgbSpace};
use matrix::{matrix_inverse, multiply_xyz_to_rgb};
use {clamp, flt};
use pixel::{RgbPixel, Srgb, GammaRgb};
use blend::PreAlpha;

///Linear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](struct.Alpha.html#Rgba).
pub type RgbaLinear<C = SrgbSpace, Wp = D65, T = f32> = Alpha<RgbLinear<C, Wp, T>, T>;

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
pub struct RgbLinear<C = Srgb, Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
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
    pub white_point: PhantomData<Wp>,

    ///The Rgb space associated with the color.
    pub rgb_space: PhantomData<C>,
}

impl<C, Wp, T> Copy for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>

{}

impl<C, Wp, T> Clone for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn clone(&self) -> RgbLinear<C, Wp, T> { *self }
}

impl<T> RgbLinear<SrgbSpace, D65, T>
    where T: Float,
{
    ///Linear RGB with white point D65.
    pub fn new(red: T, green: T, blue: T) -> RgbLinear<SrgbSpace, D65, T> {
        RgbLinear {
            red: red,
            green: green,
            blue: blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values with whtie point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> RgbLinear<SrgbSpace, D65, T> {
        RgbLinear {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            white_point: PhantomData,
            rgb_space: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    ///Linear RGB.
    pub fn with_wp(red: T, green: T, blue: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: red,
            green: green,
            blue: blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
            rgb_space: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> RgbLinear<C, Wp, T> {
        let (r, g, b, _) = pixel.to_rgba();
        RgbLinear::with_wp(r, g, b)
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
impl<T> Alpha<RgbLinear<SrgbSpace, D65, T>, T>
    where T: Float,
{
    ///Linear RGB with transparency and with white point D65.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> RgbaLinear<SrgbSpace, D65, T> {
        Alpha {
            color: RgbLinear::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values and with white point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> RgbaLinear<SrgbSpace, D65, T> {
        Alpha {
            color: RgbLinear::new_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<C, Wp, T> Alpha<RgbLinear<C, Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    ///Linear RGB with transparency.
    pub fn with_wp(red: T, green: T, blue: T, alpha: T) -> RgbaLinear<C, Wp, T> {
        Alpha {
            color: RgbLinear::with_wp(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8, alpha: u8) -> RgbaLinear<C, Wp, T> {
        Alpha {
            color: RgbLinear::with_wp_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> RgbaLinear<C, Wp, T> {
        let (r, g, b, a) = pixel.to_rgba();
        RgbaLinear::with_wp(r, g, b, a)
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
            clamp(self.alpha, T::zero(), T::one())
        )
    }
}

impl<C, Wp, T> FromColor<Wp, T> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let transform_matrix = matrix_inverse(C::rgb_to_xyz_matrix());
        multiply_xyz_to_rgb::<Wp, Wp, T>(&transform_matrix, &xyz)
    }


    fn from_rgb(rgb: RgbLinear<C, Wp, T>) -> Self {
        rgb
    }

    fn from_hsl(hsl: Hsl<Wp, T>) -> Self {
        let c = (T::one() - ( hsl.lightness * flt(2.0) - T::one()).abs()) * hsl.saturation;
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


        RgbLinear {
            red: red + m,
            green: green + m,
            blue: blue + m,
            white_point: PhantomData,
            rgb_space: PhantomData,
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


        RgbLinear {
            red: red + m,
            green: green + m,
            blue: blue + m,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }

    }

    fn from_luma(luma: Luma<Wp, T>) -> Self {
        RgbLinear {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }

}

impl<C, Wp, T> Limited for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() &&
        self.green >= T::zero() && self.green <= T::one() &&
        self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> RgbLinear<C, Wp, T> {
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

impl<C, Wp, T> Mix for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Scalar = T;

    fn mix(&self, other: &RgbLinear<C, Wp, T>, factor: T) -> RgbLinear<C, Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        RgbLinear {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Shade for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> GetHue for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = flt(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians((sqrt_3 * (self.green - self.blue)).atan2(self.red * flt(2.0) - self.green - self.blue)))
        }
    }
}

impl<C, Wp, T> Blend for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Color = RgbLinear<C, Wp, T>;

    fn into_premultiplied(self) -> PreAlpha<RgbLinear<C, Wp, T>, T> {
        RgbaLinear::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<RgbLinear<C, Wp, T>, T>) -> Self {
        RgbaLinear::from(color).into()
    }
}

impl<C, Wp, T> ComponentWise for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &RgbLinear<C, Wp, T>, mut f: F) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: f(self.red, other.red),
            green: f(self.green, other.green),
            blue: f(self.blue, other.blue),
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: f(self.red),
            green: f(self.green),
            blue: f(self.blue),
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Default for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn default() -> RgbLinear<C, Wp, T> {
        RgbLinear::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<C, Wp, T> Add<RgbLinear<C, Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn add(self, other: RgbLinear<C, Wp, T>) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Add<T> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn add(self, c: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Sub<RgbLinear<C, Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn sub(self, other: RgbLinear<C, Wp, T>) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Sub<T> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn sub(self, c: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Mul<RgbLinear<C, Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn mul(self, other: RgbLinear<C, Wp, T>) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Mul<T> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn mul(self, c: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Div<RgbLinear<C, Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn div(self, other: RgbLinear<C, Wp, T>) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> Div<T> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    type Output = RgbLinear<C, Wp, T>;

    fn div(self, c: T) -> RgbLinear<C, Wp, T> {
        RgbLinear {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            white_point: PhantomData,
            rgb_space: PhantomData,
        }
    }
}

impl<C, Wp, T> From<Srgb<Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn from(srgb: Srgb<Wp, T>) -> RgbLinear<C, Wp, T> {
        srgb.to_linear().into()
    }
}

impl<C, Wp, T> From<GammaRgb<Wp, T>> for RgbLinear<C, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        C: RgbPrimaries<Wp, T>
{
    fn from(gamma_rgb: GammaRgb<Wp, T>) -> RgbLinear<C, Wp, T> {
        gamma_rgb.to_linear().into()
    }
}

impl<C, Wp, T> From<Srgb<Wp, T>> for Alpha<RgbLinear<C, Wp, T>, T>
where T: Float,
    Wp: WhitePoint<T>,
    C: RgbPrimaries<Wp, T>
{
    fn from(srgb: Srgb<Wp, T>) -> Alpha<RgbLinear<C, Wp, T>, T> {
        srgb.to_linear()
    }
}

impl<C, Wp, T> From<GammaRgb<Wp, T>> for Alpha<RgbLinear<C, Wp, T>, T>
where T: Float,
    Wp: WhitePoint<T>,
    C: RgbPrimaries<Wp, T>
{
    fn from(gamma_rgb: GammaRgb<Wp, T>) -> Alpha<RgbLinear<C, Wp, T>, T> {
        gamma_rgb.to_linear()
    }
}

#[cfg(test)]
mod test {
    use Rgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Rgb;
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
