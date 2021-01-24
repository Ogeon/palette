//! Experimental scope.

use crate::{convert, encoding::pixel::ArrayRepr, pipeline::Pipeline, Pixel};

pub use self::{
    into_color::{IntoColor, IntoColorUnclamped},
    map::Map,
};

mod into_color;
mod map;

/// Experimental scope trait.
pub trait Scope: AsMut<[<Self as Scope>::Item]> {
    /// The item type in the scope's buffer.
    type Item: ArrayRepr + Clone;

    /// Temporarily converts a buffer, using an arbitrary conversion and restoration.
    ///
    /// ```
    /// use palette::{scope::Scope, Srgb, LinSrgb, FromColor};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut byte_scale_colors = colors.map(
    ///         |color| LinSrgb::from_color(color) * 255.0,
    ///         |color| Srgb::from_color(color / 255.0)
    ///     );
    ///
    ///     for color in &mut *byte_scale_colors {
    ///         *color -= 100.0;
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    fn map<C, R>(&mut self, convert: C, restore: R) -> Map<C::Output, R>
    where
        C: Pipeline<Self::Item>,
        C::Output: Clone,
        C::Output: ArrayRepr<
            Component = <Self::Item as Pixel>::Component,
            ArrayType = <Self::Item as ArrayRepr>::ArrayType,
        >,
        R: Pipeline<C::Output, Output = Self::Item>,
    {
        self::map::Map::new(self.as_mut(), convert, restore)
    }

    /// Temporarily converts a buffer of `T` into a buffer of `U`.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    fn into_color<C>(&mut self) -> IntoColor<Self::Item, C>
    where
        Self::Item: convert::IntoColor<C>,
        C: convert::IntoColor<Self::Item> + Clone,
        C: ArrayRepr<
            Component = <Self::Item as Pixel>::Component,
            ArrayType = <Self::Item as ArrayRepr>::ArrayType,
        >,
    {
        IntoColor::new(self.as_mut())
    }

    /// Temporarily converts a buffer of `T` into a buffer of `U`.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color_unclamped::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color_unclamped::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    fn into_color_unclamped<C>(&mut self) -> IntoColorUnclamped<Self::Item, C>
    where
        Self::Item: convert::IntoColorUnclamped<C>,
        C: convert::IntoColorUnclamped<Self::Item> + Clone,
        C: ArrayRepr<
            Component = <Self::Item as Pixel>::Component,
            ArrayType = <Self::Item as ArrayRepr>::ArrayType,
        >,
    {
        IntoColorUnclamped::new(self.as_mut())
    }
}

impl<T: ArrayRepr + Clone> Scope for [T] {
    type Item = T;
}

fn map_buffer<T, U, F>(buffer: &mut [T], convert: F) -> &mut [U]
where
    F: FnMut(T) -> U,
    T: ArrayRepr + Clone,
    U: ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    convert_in_place(buffer, convert);
    U::from_raw_slice_mut(T::into_raw_slice_mut(buffer))
}

fn convert_in_place<T, U, F>(buffer: &mut [T], mut convert: F)
where
    F: FnMut(T) -> U,
    T: ArrayRepr + Clone,
    U: ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    for color in &mut *buffer {
        let source = color.clone();
        let destination = U::from_raw_mut(color.as_raw_mut::<T::ArrayType>());
        *destination = convert(source);
    }
}

#[cfg(test)]
mod test {
    use super::Scope;
    use crate::{Hsv, LinSrgb, Shade};

    #[test]
    fn one_layer() {
        let mut colors = [LinSrgb::new(0.5f32, 0.0, 0.5), LinSrgb::new(0.5, 1.0, 0.5)];

        {
            let mut hsv_colors = colors.into_color::<Hsv<_>>();

            for color in &mut *hsv_colors {
                *color = color.darken(0.5);
            }
        }

        assert_eq!(
            &colors,
            &[
                LinSrgb::new(0.0f32, 0.0, 0.0),
                LinSrgb::new(0.25, 0.5, 0.25)
            ]
        );
    }
}
