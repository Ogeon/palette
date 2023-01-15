window.SIDEBAR_ITEMS = {"derive":[["WithAlpha",""]],"fn":[["contrast_ratio","Calculate the ratio between two `luma` values."]],"mod":[["angle","Traits for working with angular values, such as for in hues."],["blend","Color blending and blending equations."],["bool_mask","Traits for abstracting over Boolean types."],["cast","Traits and functions for casting colors to and from other data types."],["chromatic_adaptation","Convert colors from one reference white point to another"],["convert","Traits for converting between color spaces."],["encoding","Number and color encoding traits, types and standards."],["luma","Luminance types."],["named","A collection of named color constants. Can be toggled with the `\"named\"` and `\"named_from_str\"` Cargo features."],["num","Traits for abstracting over numeric types."],["rgb","RGB types, spaces and standards."],["stimulus","Traits for working with stimulus colors and values, such as RGB and XYZ."],["white_point","Defines the tristimulus values of the CIE Illuminants."]],"struct":[["Alpha","An alpha component wrapper for colors."],["FromColorMutGuard","A scope guard that restores the guarded colors to their original type when dropped."],["Hsl","HSL color space."],["Hsluv","HSLuv color space."],["Hsv","HSV color space."],["Hwb","HWB color space."],["Lab","The CIE L*a*b* (CIELAB) color space."],["LabHue","A hue type for the CIE L*a*b* family of color spaces."],["Lch","CIE L*C*h°, a polar version of CIE L*a*b*."],["Lchuv","CIE L*C*uv h°uv, a polar version of CIE L*u*v*."],["Luv","The CIE L*u*v* (CIELUV) color space."],["LuvHue","A hue type for the CIE L*u*v* family of color spaces."],["Okhsl","A Hue/Saturation/Lightness representation of [`Oklab`] in the `sRGB` color space."],["Okhsv","A Hue/Saturation/Value representation of [`Oklab`] in the `sRGB` color space."],["Okhwb","A Hue/Whiteness/Blackness representation of [`Oklab`][crate::Oklab] in the `sRGB` color space, similar to [`Hwb`][crate::Okhwb]."],["Oklab","The Oklab color space."],["OklabHue","A hue type for the Oklab color space."],["Oklch","Oklch, a polar version of Oklab."],["RgbHue","A hue type for the RGB family of color spaces."],["Xyz","The CIE 1931 XYZ color space."],["Yxy","The CIE 1931 Yxy (xyY)  color space."]],"trait":[["ArrayExt","Extension trait for fixed size arrays."],["Clamp","An operator for restricting a color’s components to their expected ranges."],["ClampAssign","An assigning operator for restricting a color’s components to their expected ranges."],["ColorDifference","A trait for calculating the color difference between two colors."],["Darken","Operators for darkening a color;"],["DarkenAssign","Assigning operators for darkening a color;"],["Desaturate","Operator for decreasing the saturation (or chroma) of a color."],["DesaturateAssign","Assigning operator for decreasing the saturation (or chroma) of a color."],["FromColor","A trait for converting one color from another, in a possibly lossy way."],["FromColorMut","Temporarily convert colors in place."],["GetHue","A trait for colors where a hue may be calculated."],["IntoColor","A trait for converting a color into another, in a possibly lossy way."],["IntoColorMut","Temporarily convert colors in place. The `Into` counterpart to [`FromColorMut`]."],["IsWithinBounds","Checks if color components are within their expected range bounds."],["Lighten","Operators for lightening a color."],["LightenAssign","Assigning operators for lightening a color."],["Mix","Linear color interpolation of two colors."],["MixAssign","Assigning linear color interpolation of two colors."],["NextArray","Temporary helper trait for getting an array type of size `N + 1`."],["RelativeContrast","A trait for calculating relative contrast between two colors."],["Saturate","Operator for increasing the saturation (or chroma) of a color."],["SaturateAssign","Assigning operator for increasing the saturation (or chroma) of a color."],["SetHue","Change the hue of a color to a specific value without moving."],["ShiftHue","Operator for increasing or decreasing the hue by an amount."],["ShiftHueAssign","Assigning operator for increasing or decreasing the hue by an amount."],["WithAlpha","A trait for color types that can have or be given transparency (alpha channel)."],["WithHue","Change the hue of a color to a specific value."]],"type":[["Hsla","Linear HSL with an alpha component. See the `Hsla` implementation in `Alpha`."],["Hsluva","HSLuv with an alpha component. See the `Hsluva` implementation in `Alpha`."],["Hsva","Linear HSV with an alpha component. See the `Hsva` implementation in `Alpha`."],["Hwba","Linear HWB with an alpha component. See the `Hwba` implementation in `Alpha`."],["Laba","CIE L*a*b* (CIELAB) with an alpha component. See the `Laba` implementation in `Alpha`."],["Lcha","CIE L*C*h° with an alpha component. See the `Lcha` implementation in `Alpha`."],["Lchuva","CIE L*C*uv h°uv with an alpha component. See the `Lchuva` implementation in `Alpha`."],["Luva","CIE L*u*v* (CIELUV) with an alpha component. See the `Luva` implementation in `Alpha`."],["Mat3","A 9 element array representing a 3x3 matrix."],["Okhsla","Okhsl with an alpha component."],["Okhsva","Okhsv with an alpha component. See the `Okhsva` implementation in `Alpha`."],["Okhwba","Okhwb with an alpha component. See the `Okhwba` implementation in `Alpha`."],["Oklaba","Oklab with an alpha component."],["Oklcha","Oklch with an alpha component. See the `Oklcha` implementation in `Alpha`."],["Xyza","CIE 1931 XYZ with an alpha component. See the `Xyza` implementation in `Alpha`."],["Yxya","CIE 1931 Yxy (xyY) with an alpha component. See the `Yxya` implementation in `Alpha`."]]};