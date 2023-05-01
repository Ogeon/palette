#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::num::{Cbrt, Powi, Sqrt};
use crate::OklabHue;

use super::Okhsv;

#[cfg(feature = "random")]
impl<T> Distribution<Okhsv<T>> for Standard
where
    T: Cbrt + Sqrt,
    Standard: Distribution<T> + Distribution<OklabHue<T>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhsv<T> {
        crate::random_sampling::sample_okhsv(rng.gen::<OklabHue<T>>(), rng.gen(), rng.gen())
    }
}

/// Sample Okhsv colors uniformly.
#[cfg(feature = "random")]
pub struct UniformOkhsv<T>
where
    T: SampleUniform,
{
    hue: crate::hues::UniformOklabHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Okhsv<T>
where
    T: Cbrt + Sqrt + Powi + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type Sampler = UniformOkhsv<T>;
}

#[cfg(feature = "random")]
impl<T> UniformSampler for UniformOkhsv<T>
where
    T: Cbrt + Sqrt + Powi + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type X = Okhsv<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        let (r1_min, r2_min) = (low.value.powi(3), low.saturation.powi(2));
        let (r1_max, r2_max) = (high.value.powi(3), high.saturation.powi(2));

        UniformOkhsv {
            hue: crate::hues::UniformOklabHue::new(low.hue, high.hue),
            u1: Uniform::new::<_, T>(r1_min, r1_max),
            u2: Uniform::new::<_, T>(r2_min, r2_max),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        let (r1_min, r2_min) = (low.value.powi(3), low.saturation.powi(2));
        let (r1_max, r2_max) = (high.value.powi(3), high.saturation.powi(2));

        UniformOkhsv {
            hue: crate::hues::UniformOklabHue::new_inclusive(low.hue, high.hue),
            u1: Uniform::new_inclusive::<_, T>(r1_min, r1_max),
            u2: Uniform::new_inclusive::<_, T>(r2_min, r2_max),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhsv<T> {
        crate::random_sampling::sample_okhsv(
            self.hue.sample(rng),
            self.u1.sample(rng),
            self.u2.sample(rng),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Okhsv<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Okhsv<T> where T: bytemuck::Pod {}
