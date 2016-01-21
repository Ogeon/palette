macro_rules! make_color {
    ($(
        #[$variant_comment:meta]
        $variant:ident {$(
            #[$ctor_comment:meta]
            $ctor_name:ident $( <$( $ty_params:ident: $ty_param_traits:ident $( <$( $ty_inner_traits:ident ),*> )*),*> )* ($($ctor_field:ident : $ctor_ty:ty),*);
        )+}
    )+) => (

        ///A generic color type.
        ///
        ///The `Color` may belong to any color space and it may change
        ///depending on which operation is performed. That makes it immune to
        ///the "without conversion" rule of the operations it supports. The
        ///color spaces are selected as follows:
        ///
        /// * `Mix`: RGB for no particular reason, except that it's intuitive.
        /// * `Shade`: CIE L*a*b* for its luminance component.
        /// * `Hue` and `GetHue`: CIE L*C*h° for its hue component and how it preserves the apparent lightness.
        /// * `Saturate`: CIE L*C*h° for its chromaticity component and how it preserves the apparent lightness.
        ///
        ///It's not recommended to use `Color` when full control is necessary,
        ///but it can easily be converted to a fixed color space in those
        ///cases.
        #[derive(Clone, Copy, Debug)]
        pub enum Color<T:Float> {
            $(#[$variant_comment] $variant($variant<T>)),+
        }

        $(
            impl<T:Float> Color<T> {
                $(
                    #[$ctor_comment]
                    pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty),*) -> Color<T> {
                        Color::$variant($variant::$ctor_name($($ctor_field),*))
                    }
                )+
            }
        )+

        impl<T:Float> Mix for Color<T> {
            fn mix(&self, other: &Color<T>, factor: T) -> Color<T> {
                Rgb::from(*self).mix(&Rgb::from(*other), factor).into()
            }
        }

        impl<T:Float> Shade for Color<T> {
            fn lighten(&self, amount: T) -> Color<T> {
                Lab::from(*self).lighten(amount).into()
            }
        }

        impl<T:Float> GetHue for Color<T> {
            type Hue = LabHue<T>;

            fn get_hue(&self) -> Option<LabHue<T>> {
                Lch::from(*self).get_hue()
            }
        }

        impl<T:Float> Hue for Color<T> {
            fn with_hue(&self, hue: LabHue<T>) -> Color<T> {
                Lch::from(*self).with_hue(hue).into()
            }

            fn shift_hue(&self, amount: LabHue<T>) -> Color<T> {
                Lch::from(*self).shift_hue(amount).into()
            }
        }

        impl<T:Float> Saturate for Color<T> {
            fn saturate(&self, factor: T) -> Color<T> {
                Lch::from(*self).saturate(factor).into()
            }
        }

        $(
            impl<T:Float> From<$variant<T>> for Color<T> {
                fn from(color: $variant<T>) -> Color<T> {
                    Color::$variant(color)
                }
            }
        )+
    )
}




make_color! {
    ///Linear luminance.
    Luma {
        ///Linear luminance.
        y(luma: T);

        ///Linear luminance with transparency.
        ya(luma: T, alpha: T);

        ///Linear luminance from an 8 bit value.
        y8(luma: u8);

        ///Linear luminance and transparency from 8 bit values.
        ya8(luma: u8, alpha: u8);
    }

    ///Linear RGB.
    Rgb {
        ///Linear RGB.
        linear_rgb(red: T, green: T, blue: T);

        ///Linear RGB and transparency.
        linear_rgba(red: T, green: T, blue: T, alpha: T);

        ///Linear RGB from 8 bit values.
        linear_rgb8(red: u8, green: u8, blue: u8);

        ///Linear RGB and transparency from 8 bit values.
        linear_rgba8(red: u8, green: u8, blue: u8, alpha: u8);

        ///Linear RGB from a linear pixel value.
        linear_pixel<P: RgbPixel<T> >(pixel: &P);

        ///Linear RGB from sRGB.
        srgb(red: T, green: T, blue: T);

        ///Linear RGB from sRGB with transparency.
        srgba(red: T, green: T, blue: T, alpha: T);

        ///Linear RGB from 8 bit sRGB.
        srgb8(red: u8, green: u8, blue: u8);

        ///Linear RGB from 8 bit sRGB with transparency.
        srgba8(red: u8, green: u8, blue: u8, alpha: u8);

        ///Linear RGB from an sRGB pixel value.
        srgb_pixel<P: RgbPixel<T> >(pixel: &P);

        ///Linear RGB from gamma corrected RGB.
        gamma_rgb(red: T, green: T, blue: T, gamma: T);

        ///Linear RGB from gamma corrected RGB with transparency.
        gamma_rgba(red: T, green: T, blue: T, alpha: T, gamma: T);

        ///Linear RGB from 8 bit gamma corrected RGB.
        gamma_rgb8(red: u8, green: u8, blue: u8, gamma: T);

        ///Linear RGB from 8 bit gamma corrected RGB with transparency.
        gamma_rgba8(red: u8, green: u8, blue: u8, alpha: u8, gamma: T);

        ///Linear RGB from a gamma corrected pixel value.
        gamma_pixel<P: RgbPixel<T> >(pixel: &P, gamma: T);
    }

    ///CIE 1931 XYZ.
    Xyz {
        ///CIE XYZ.
        xyz(x: T, y: T, z: T);

        ///CIE XYZ and transparency.
        xyza(x: T, y: T, z: T, alpha: T);
    }

    ///CIE L*a*b* (CIELAB).
    Lab {
        ///CIE L*a*b*.
        lab(l: T, a: T, b: T);

        ///CIE L*a*b* and transparency.
        laba(l: T, a: T, b: T, alpha: T);
    }

    ///CIE L*C*h°, a polar version of CIE L*a*b*.
    Lch {
        ///CIE L*C*h°.
        lch(l: T, chroma: T, hue: LabHue<T>);

        ///CIE L*C*h° and transparency.
        lcha(l: T, chroma: T, hue: LabHue<T>, alpha: T);
    }

    ///Linear HSV, a cylindrical version of RGB.
    Hsv {
        ///Linear HSV.
        hsv(hue: RgbHue<T>, saturation: T, value: T);

        ///Linear HSV and transparency.
        hsva(hue: RgbHue<T>, saturation: T, value: T, alpha: T);
    }

    ///Linear HSL, a cylindrical version of RGB.
    Hsl {
        ///Linear HSL.
        hsl(hue: RgbHue<T>, saturation: T, lightness: T);

        ///Linear HSL and transparency.
        hsla(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T);
    }
}
