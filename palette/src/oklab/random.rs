use super::Oklab;
use crate::num::Real;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;
use std::ops::{Mul, Sub};

impl<T> Distribution<Oklab<T>> for Standard
where
    T: Real + Mul<Output = T> + Sub<Output = T>,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-1.0, 1.0)
    // TODO: the choice for a and b is random and rather bad:
    //  1. a and b are unlimited. Oklab can express the whole electro-magnetic spectrum
    //  2. Oklab is a perceptual color space. It would make sense to limit random
    //  values to the limits of human perception.
    //  https://bottosson.github.io/posts/oklab/#luo-rigg-dataset-and-full-gamut
    //  shows, that at least for some L, a should not be greater than 0.5, to
    //  avoid leaving the perceivable gamut. Though it could go to -2.5.
    //  3. If people want random sRGB values: Expressing the sRGB bounds in Oklab is
    //  beyond my abilities. However, it is not necessary either. It can be done in
    //  Okhsv, Okhsl or Okhwb. Maybe we should  not offer the ability in Oklab and
    //  encourage sampling in the color spaces that are limited to sRGB gamut.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(
            rng.gen(),
            rng.gen() * T::from_f64(2.0) - T::from_f64(1.0),
            rng.gen() * T::from_f64(2.0) - T::from_f64(1.0),
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
