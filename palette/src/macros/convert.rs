/// Check that traits for converting to and from XYZ have been implemented.
#[cfg(test)]
macro_rules! test_convert_into_from_xyz {
    ($ty:ty) => {
        #[test]
        fn convert_from_xyz() {
            use crate::FromColor;

            let _: $ty = <$ty>::from_color(crate::Xyz::default());
        }

        #[test]
        fn convert_into_xyz() {
            use crate::FromColor;

            let _: crate::Xyz = crate::Xyz::from_color(<$ty>::default());
        }
    };
}
