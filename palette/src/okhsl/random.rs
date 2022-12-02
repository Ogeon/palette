use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::num::{Cbrt, Powi, Sqrt};
use crate::{
    bool_mask::LazySelect,
    num::{Arithmetics, One, PartialCmp, Real},
    OklabHue,
};

use super::Okhsl;

impl<T> Distribution<Okhsl<T>> for Standard
where
    T: Real + One + Cbrt + Sqrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
    Standard: Distribution<T> + Distribution<OklabHue<T>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhsl<T> {
        crate::random_sampling::sample_okhsl(rng.gen::<OklabHue<T>>(), rng.gen(), rng.gen())
    }
}

pub struct UniformOkhsl<T>
where
    T: SampleUniform,
{
    hue: crate::hues::UniformOklabHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
}

impl<T> SampleUniform for Okhsl<T>
where
    T: Real + One + Cbrt + Sqrt + Powi + Arithmetics + PartialCmp + Clone + SampleUniform,
    T::Mask: LazySelect<T> + Clone,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type Sampler = UniformOkhsl<T>;
}

impl<T> UniformSampler for UniformOkhsl<T>
where
    T: Real + One + Cbrt + Sqrt + Powi + Arithmetics + PartialCmp + Clone + SampleUniform,
    T::Mask: LazySelect<T> + Clone,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type X = Okhsl<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        use crate::random_sampling::invert_hsl_sample;

        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        let (r1_min, r2_min) = invert_hsl_sample(low.saturation, low.lightness);
        let (r1_max, r2_max) = invert_hsl_sample(high.saturation, high.lightness);

        UniformOkhsl {
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
        use crate::random_sampling::invert_hsl_sample;

        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        let (r1_min, r2_min) = invert_hsl_sample(low.saturation, low.lightness);
        let (r1_max, r2_max) = invert_hsl_sample(high.saturation, high.lightness);

        UniformOkhsl {
            hue: crate::hues::UniformOklabHue::new_inclusive(low.hue, high.hue),
            u1: Uniform::new_inclusive::<_, T>(r1_min, r1_max),
            u2: Uniform::new_inclusive::<_, T>(r2_min, r2_max),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhsl<T> {
        crate::random_sampling::sample_okhsl(
            self.hue.sample(rng),
            self.u1.sample(rng),
            self.u2.sample(rng),
        )
    }
}
