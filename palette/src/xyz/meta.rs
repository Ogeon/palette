//! Meta types and traits for [`Xyz`][super::Xyz].

/// Implemented by meta types that contain a meta type for [`Xyz`][super::Xyz].
pub trait HasXyzMeta {
    /// A meta type that can be used in [`Xyz`][super::Xyz].
    type XyzMeta;
}
