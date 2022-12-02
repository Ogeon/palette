use super::Oklab;
use crate::num::One;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;
use std::ops::{Add, Mul, Sub};

impl<T> Distribution<Oklab<T>> for Standard
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + One,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-1.0, 1.0)
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(
            rng.gen(),
            rng.gen() * (T::one() + T::one()) - T::one(),
            rng.gen() * (T::one() + T::one()) - T::one(),
        )
    }
}

pub struct UniformOklab<T>
where
    T: SampleUniform,
{
    l: Uniform<T>,
    a: Uniform<T>,
    b: Uniform<T>,
}

impl<T> UniformSampler for UniformOklab<T>
where
    T: Clone + SampleUniform,
{
    type X = Oklab<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        Self {
            l: Uniform::new::<_, T>(low.l, high.l),
            a: Uniform::new::<_, T>(low.a, high.a),
            b: Uniform::new::<_, T>(low.b, high.b),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        Self {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            a: Uniform::new_inclusive::<_, T>(low.a, high.a),
            b: Uniform::new_inclusive::<_, T>(low.b, high.b),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(self.l.sample(rng), self.a.sample(rng), self.b.sample(rng))
    }
}

impl<T> SampleUniform for Oklab<T>
where
    T: Clone + SampleUniform,
{
    type Sampler = UniformOklab<T>;
}
