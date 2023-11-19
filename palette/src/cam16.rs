//! Types for the CIE CAM16 color appearance model.

pub use full::*;
pub use parameters::*;
pub use partial::*;
pub use ucs_jab::{Cam16UcsJab, Cam16UcsJaba, Iter as Cam16UcsJabIter};
pub use ucs_jmh::{Cam16UcsJmh, Cam16UcsJmha, Iter as Cam16UcsJmhIter};

#[cfg(feature = "random")]
pub use ucs_jab::UniformCam16UcsJab;
#[cfg(feature = "random")]
pub use ucs_jmh::UniformCam16UcsJmh;

mod full;
mod math;
mod parameters;
mod partial;
mod ucs_jab;
mod ucs_jmh;
