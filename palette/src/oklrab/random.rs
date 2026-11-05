use core::ops::{Add, Mul, Sub};

use crate::{num::One, Oklrab};

impl_rand_traits_cartesian!(
    UniformOklrab,
    Oklrab {
        l,
        a => [|x| x  * (T::one() + T::one()) - T::one()],
        b => [|x| x  * (T::one() + T::one()) - T::one()]
    }
    where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + One
);
