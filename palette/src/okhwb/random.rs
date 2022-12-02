#[cfg(feature = "approx")]
use crate::{convert::FromColorUnclamped, Okhsv};

use super::Okhwb;
use crate::num::MinMax;
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

impl<T> Distribution<Okhwb<T>> for Standard
where
    Standard: Distribution<Okhsv<T>>,
    Okhwb<T>: FromColorUnclamped<Okhsv<T>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhwb<T> {
        Okhwb::from_color_unclamped(rng.gen::<Okhsv<T>>())
    }
}

pub struct UniformOkhwb<T>
where
    T: SampleUniform,
{
    sampler: crate::okhsv::UniformOkhsv<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Okhwb<T>
where
    T: MinMax + Clone + SampleUniform,
    Okhsv<T>: FromColorUnclamped<Okhwb<T>> + SampleBorrow<Okhsv<T>>,
    Okhwb<T>: FromColorUnclamped<Okhsv<T>>,
    crate::okhsv::UniformOkhsv<T>: UniformSampler<X = Okhsv<T>>,
{
    type Sampler = UniformOkhwb<T>;
}

impl<T> UniformSampler for UniformOkhwb<T>
where
    T: MinMax + Clone + SampleUniform,
    Okhsv<T>: FromColorUnclamped<Okhwb<T>> + SampleBorrow<Okhsv<T>>,
    Okhwb<T>: FromColorUnclamped<Okhsv<T>>,
    crate::okhsv::UniformOkhsv<T>: UniformSampler<X = Okhsv<T>>,
{
    type X = Okhwb<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low_input = Okhsv::from_color_unclamped(low_b.borrow().clone());
        let high_input = Okhsv::from_color_unclamped(high_b.borrow().clone());

        let (low_saturation, high_saturation) = low_input.saturation.min_max(high_input.saturation);
        let (low_value, high_value) = low_input.value.min_max(high_input.value);

        let low = Okhsv::new(low_input.hue, low_saturation, low_value);
        let high = Okhsv::new(high_input.hue, high_saturation, high_value);

        let sampler = crate::okhsv::UniformOkhsv::new(low, high);

        UniformOkhwb { sampler }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low_input = Okhsv::from_color_unclamped(low_b.borrow().clone());
        let high_input = Okhsv::from_color_unclamped(high_b.borrow().clone());

        let (low_saturation, high_saturation) = low_input.saturation.min_max(high_input.saturation);
        let (low_value, high_value) = low_input.value.min_max(high_input.value);

        let low = Okhsv::new(low_input.hue, low_saturation, low_value);
        let high = Okhsv::new(high_input.hue, high_saturation, high_value);

        let sampler = crate::okhsv::UniformOkhsv::new_inclusive(low, high);

        UniformOkhwb { sampler }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Okhwb<T> {
        Okhwb::from_color_unclamped(self.sampler.sample(rng))
    }
}
