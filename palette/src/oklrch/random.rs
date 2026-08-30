use super::Oklrch;

use crate::OklabHue;

impl_rand_traits_cylinder!(
    UniformOklrch,
    Oklrch {
        hue: UniformOklabHue => OklabHue,
        height: l,
        radius: chroma // FIXME: Same as with Oklrab: The limit of chroma has no meaning
    }
);
