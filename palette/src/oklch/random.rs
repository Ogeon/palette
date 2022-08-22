use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::num::{Real, Sqrt};

use super::Oklch;

use crate::OklabHue;

impl<T> Distribution<Oklch<T>> for Standard
where
    T: Real + Sqrt,

    Standard: Distribution<T> + Distribution<OklabHue<T>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        // FIXME: Same as with Oklab: The limit of chroma has no meaning
        Oklch::new(rng.gen(), rng.gen::<T>().sqrt(), rng.gen::<OklabHue<T>>())
    }
}

pub struct UniformOklch<T>
where
    T: SampleUniform,
{
    l: Uniform<T>,
    chroma: Uniform<T>,
    hue: crate::hues::UniformOklabHue<T>,
}

impl<T> SampleUniform for Oklch<T>
where
    T: Sqrt + core::ops::Mul<Output = T> + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type Sampler = UniformOklch<T>;
}

impl<T> UniformSampler for UniformOklch<T>
where
    T: Sqrt + core::ops::Mul<Output = T> + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type X = Oklch<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformOklch {
            l: Uniform::new::<_, T>(low.l, high.l),
            chroma: Uniform::new::<_, T>(
                low.chroma.clone() * low.chroma,
                high.chroma.clone() * high.chroma,
            ),
            hue: crate::hues::UniformOklabHue::new(low.hue, high.hue),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformOklch {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            chroma: Uniform::new_inclusive::<_, T>(
                low.chroma.clone() * low.chroma,
                high.chroma.clone() * high.chroma,
            ),
            hue: crate::hues::UniformOklabHue::new_inclusive(low.hue, high.hue),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        Oklch {
            l: self.l.sample(rng),
            chroma: self.chroma.sample(rng).sqrt(),
            hue: self.hue.sample(rng),
        }
    }
}
