use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use IntoColor;
use flt;
use num::{Float, FromPrimitive, Zero};
use ordered_float::OrderedFloat;
use white_point::WhitePoint;
use xyz::Xyz;

/// The smallest wavelength (in nanometres) represented 
/// in the `Spectrum` data structure. This is the first 
/// value in the map's wavelength.
pub const MIN_WAVELENGTH: usize = 360;

/// The largest wavelength (in nanometres) represented
/// in the `Spectrum` data structure. This is the last
/// value in the map's wavelength.
#[allow(dead_code)]
pub const MAX_WAVELENGTH: usize = 830;

const SAMPLE_STEP: usize = 5;

pub const SAMPLE_COUNT: usize = (MAX_WAVELENGTH - MIN_WAVELENGTH) / SAMPLE_STEP + 1;

/// Lookup table for converting a wavelength in the range
/// [360, 830] nm into XYZ tristimulus values.
fn spectrum_to_xyz_map<T: Float>() -> [(T, T, T); SAMPLE_COUNT] {
    [
        (flt(0.000129900000),flt(0.000003917000),flt(0.000606100000)),
        (flt(0.000232100000),flt(0.000006965000),flt(0.001086000000)),
        (flt(0.000414900000),flt(0.000012390000),flt(0.001946000000)),
        (flt(0.000741600000),flt(0.000022020000),flt(0.003486000000)),
        (flt(0.001368000000),flt(0.000039000000),flt(0.006450001000)),
        (flt(0.002236000000),flt(0.000064000000),flt(0.010549990000)),
        (flt(0.004243000000),flt(0.000120000000),flt(0.020050010000)),
        (flt(0.007650000000),flt(0.000217000000),flt(0.036210000000)),
        (flt(0.014310000000),flt(0.000396000000),flt(0.067850010000)),
        (flt(0.023190000000),flt(0.000640000000),flt(0.110200000000)),
        (flt(0.043510000000),flt(0.001210000000),flt(0.207400000000)),
        (flt(0.077630000000),flt(0.002180000000),flt(0.371300000000)),
        (flt(0.134380000000),flt(0.004000000000),flt(0.645600000000)),
        (flt(0.214770000000),flt(0.007300000000),flt(1.039050100000)),
        (flt(0.283900000000),flt(0.011600000000),flt(1.385600000000)),
        (flt(0.328500000000),flt(0.016840000000),flt(1.622960000000)),
        (flt(0.348280000000),flt(0.023000000000),flt(1.747060000000)),
        (flt(0.348060000000),flt(0.029800000000),flt(1.782600000000)),
        (flt(0.336200000000),flt(0.038000000000),flt(1.772110000000)),
        (flt(0.318700000000),flt(0.048000000000),flt(1.744100000000)),
        (flt(0.290800000000),flt(0.060000000000),flt(1.669200000000)),
        (flt(0.251100000000),flt(0.073900000000),flt(1.528100000000)),
        (flt(0.195360000000),flt(0.090980000000),flt(1.287640000000)),
        (flt(0.142100000000),flt(0.112600000000),flt(1.041900000000)),
        (flt(0.095640000000),flt(0.139020000000),flt(0.812950100000)),
        (flt(0.057950010000),flt(0.169300000000),flt(0.616200000000)),
        (flt(0.032010000000),flt(0.208020000000),flt(0.465180000000)),
        (flt(0.014700000000),flt(0.258600000000),flt(0.353300000000)),
        (flt(0.004900000000),flt(0.323000000000),flt(0.272000000000)),
        (flt(0.002400000000),flt(0.407300000000),flt(0.212300000000)),
        (flt(0.009300000000),flt(0.503000000000),flt(0.158200000000)),
        (flt(0.029100000000),flt(0.608200000000),flt(0.111700000000)),
        (flt(0.063270000000),flt(0.710000000000),flt(0.078249990000)),
        (flt(0.109600000000),flt(0.793200000000),flt(0.057250010000)),
        (flt(0.165500000000),flt(0.862000000000),flt(0.042160000000)),
        (flt(0.225749900000),flt(0.914850100000),flt(0.029840000000)),
        (flt(0.290400000000),flt(0.954000000000),flt(0.020300000000)),
        (flt(0.359700000000),flt(0.980300000000),flt(0.013400000000)),
        (flt(0.433449900000),flt(0.994950100000),flt(0.008749999000)),
        (flt(0.512050100000),flt(1.000000000000),flt(0.005749999000)),
        (flt(0.594500000000),flt(0.995000000000),flt(0.003900000000)),
        (flt(0.678400000000),flt(0.978600000000),flt(0.002749999000)),
        (flt(0.762100000000),flt(0.952000000000),flt(0.002100000000)),
        (flt(0.842500000000),flt(0.915400000000),flt(0.001800000000)),
        (flt(0.916300000000),flt(0.870000000000),flt(0.001650001000)),
        (flt(0.978600000000),flt(0.816300000000),flt(0.001400000000)),
        (flt(1.026300000000),flt(0.757000000000),flt(0.001100000000)),
        (flt(1.056700000000),flt(0.694900000000),flt(0.001000000000)),
        (flt(1.062200000000),flt(0.631000000000),flt(0.000800000000)),
        (flt(1.045600000000),flt(0.566800000000),flt(0.000600000000)),
        (flt(1.002600000000),flt(0.503000000000),flt(0.000340000000)),
        (flt(0.938400000000),flt(0.441200000000),flt(0.000240000000)),
        (flt(0.854449900000),flt(0.381000000000),flt(0.000190000000)),
        (flt(0.751400000000),flt(0.321000000000),flt(0.000100000000)),
        (flt(0.642400000000),flt(0.265000000000),flt(0.000049999990)),
        (flt(0.541900000000),flt(0.217000000000),flt(0.000030000000)),
        (flt(0.447900000000),flt(0.175000000000),flt(0.000020000000)),
        (flt(0.360800000000),flt(0.138200000000),flt(0.000010000000)),
        (flt(0.283500000000),flt(0.107000000000),flt(0.000000000000)),
        (flt(0.218700000000),flt(0.081600000000),flt(0.000000000000)),
        (flt(0.164900000000),flt(0.061000000000),flt(0.000000000000)),
        (flt(0.121200000000),flt(0.044580000000),flt(0.000000000000)),
        (flt(0.087400000000),flt(0.032000000000),flt(0.000000000000)),
        (flt(0.063600000000),flt(0.023200000000),flt(0.000000000000)),
        (flt(0.046770000000),flt(0.017000000000),flt(0.000000000000)),
        (flt(0.032900000000),flt(0.011920000000),flt(0.000000000000)),
        (flt(0.022700000000),flt(0.008210000000),flt(0.000000000000)),
        (flt(0.015840000000),flt(0.005723000000),flt(0.000000000000)),
        (flt(0.011359160000),flt(0.004102000000),flt(0.000000000000)),
        (flt(0.008110916000),flt(0.002929000000),flt(0.000000000000)),
        (flt(0.005790346000),flt(0.002091000000),flt(0.000000000000)),
        (flt(0.004109457000),flt(0.001484000000),flt(0.000000000000)),
        (flt(0.002899327000),flt(0.001047000000),flt(0.000000000000)),
        (flt(0.002049190000),flt(0.000740000000),flt(0.000000000000)),
        (flt(0.001439971000),flt(0.000520000000),flt(0.000000000000)),
        (flt(0.000999949300),flt(0.000361100000),flt(0.000000000000)),
        (flt(0.000690078600),flt(0.000249200000),flt(0.000000000000)),
        (flt(0.000476021300),flt(0.000171900000),flt(0.000000000000)),
        (flt(0.000332301100),flt(0.000120000000),flt(0.000000000000)),
        (flt(0.000234826100),flt(0.000084800000),flt(0.000000000000)),
        (flt(0.000166150500),flt(0.000060000000),flt(0.000000000000)),
        (flt(0.000117413000),flt(0.000042400000),flt(0.000000000000)),
        (flt(0.000083075270),flt(0.000030000000),flt(0.000000000000)),
        (flt(0.000058706520),flt(0.000021200000),flt(0.000000000000)),
        (flt(0.000041509940),flt(0.000014990000),flt(0.000000000000)),
        (flt(0.000029353260),flt(0.000010600000),flt(0.000000000000)),
        (flt(0.000020673830),flt(0.000007465700),flt(0.000000000000)),
        (flt(0.000014559770),flt(0.000005257800),flt(0.000000000000)),
        (flt(0.000010253980),flt(0.000003702900),flt(0.000000000000)),
        (flt(0.000007221456),flt(0.000002607800),flt(0.000000000000)),
        (flt(0.000005085868),flt(0.000001836600),flt(0.000000000000)),
        (flt(0.000003581652),flt(0.000001293400),flt(0.000000000000)),
        (flt(0.000002522525),flt(0.000000910930),flt(0.000000000000)),
        (flt(0.000001776509),flt(0.000000641530),flt(0.000000000000)),
        (flt(0.000001251141),flt(0.000000451810),flt(0.000000000000)),
    ]
}

#[inline(always)]
fn lerp<T: Float>(a: T, b: T, t: T) -> T {
    a + (b - a) * t
}

/// A sampling of a Spectral Power Distribution at a
/// particular wavelength and relative intensity.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Sample<T: Float>(OrderedFloat<f32>, T);

impl<T: Float> Sample<T> {

    /// Create a new Sample.
    ///
    /// # Panics
    /// If the intensity is less than zero.
    pub fn new(wavelength: f32, intensity: T) -> Sample<T> {
        assert!(intensity >= T::zero(), "Intensity must be >= 0.");
        Sample(OrderedFloat(wavelength), intensity)
    }

    #[inline(always)]
    pub fn wavelength(&self) -> f32 {
        self.0.into_inner()
    }

    #[inline(always)]
    pub fn intensity(&self) -> T {
        self.1
    }
}

/// Represents the relative intensity of
/// light at wavelengths from 360 nm to 830 nm.
pub struct Spectrum<T: Float> {
    data: [T; SAMPLE_COUNT],
}

impl<T: Float> Clone for Spectrum<T> {
    fn clone(&self) -> Spectrum<T> {
        let mut data: [T; SAMPLE_COUNT] = [T::zero(); SAMPLE_COUNT];
        data.copy_from_slice(&self.data);
        Spectrum {
            data: data,
        }
    }
}

impl<T> Spectrum<T>
    where T: Float + Zero + FromPrimitive
{
    /// Create a `Spectrum` from an array of spectral intensity
    /// values assumed to be in the range 360 nm to 830 nm.
    ///
    /// The intensity values must be greater than or equal to
    /// zero and not NaN.
    ///
    /// # Panics
    /// If an intensity value is less than zero.
    pub fn new(data: [T; SAMPLE_COUNT]) -> Spectrum<T> {
        assert!(data.iter().all(|&intensity| intensity >= T::zero()));
        Spectrum { data: data }
    }

    /// Create a `Spectrum` from a slice of `Sample`s.
    /// 
    /// The more Samples there are the more accurate
    /// `Spectrum`'s internal representation will be.
    ///
    /// The intensity values must be greater than or equal to
    /// zero and not NaN.
    pub fn from_samples(data: &[Sample<T>]) -> Spectrum<T> {
        // TODO: replace this with sort_unstable_by() when stabilised.
        assert!(data.iter().all(|&sample| sample.intensity() >= T::zero()));
        if data.is_empty() {
            let intensity = data.first().unwrap_or(&Sample::new(0.0, T::zero())).intensity();
            return Spectrum {
                data: [intensity; SAMPLE_COUNT]
            };
        }

        let mut data: Vec<Sample<T>> = data.iter().cloned().collect();
        data.sort_by(|a, b| a.0.cmp(&b.0));

        let mut sampled: [T; SAMPLE_COUNT] = [T::zero(); SAMPLE_COUNT];
        for ((segment_lo, segment_hi), sample) in (0..SAMPLE_COUNT).map(|i| {
                let lambda = (MIN_WAVELENGTH + (i * SAMPLE_STEP)) as f32;
                (lambda, lambda + SAMPLE_STEP as f32)
            }).zip(sampled.iter_mut())
        {
            if segment_lo >= data.last().unwrap().wavelength() {
                *sample = data.last().unwrap().intensity();
            } else if segment_hi <= data.first().unwrap().wavelength() {
                *sample = data.first().unwrap().intensity();
            } else {
                // Calculate the weighted average of contributions across
                // the segment's range given by the supplied data samples.
                let mut sum = if segment_lo < data.first().unwrap().wavelength() {
                    data.first().unwrap().intensity() * flt(data.first().unwrap().wavelength() - segment_lo)
                } else if segment_hi > data.last().unwrap().wavelength() {
                    data.last().unwrap().intensity() * flt(segment_hi - data.last().unwrap().wavelength())
                } else {
                    T::zero()
                };
                for patch in data.windows(2) {
                    let lo = patch[0];
                    let hi = patch[1];
                    if segment_hi >= lo.wavelength() && segment_lo <= hi.wavelength() {
                        let lo_wavelength = lo.wavelength().max(segment_lo);
                        let hi_wavelength = hi.wavelength().min(segment_hi);
                        let range = hi.wavelength() - lo.wavelength();
                        let lo_intensity = lerp(lo.intensity(), hi.intensity(), flt((lo_wavelength - lo.wavelength()) / range));
                        let hi_intensity = lerp(lo.intensity(), hi.intensity(), flt((hi_wavelength - lo.wavelength()) / range));
                        let avg = (lo_intensity + hi_intensity) / flt(2.0);
                        sum = sum + avg * flt(hi_wavelength - lo_wavelength);
                    }
                }
                *sample = sum / flt(segment_hi - segment_lo);
            };
        }
        Spectrum { data: sampled }
    }
}

impl<Wp, T> IntoColor<Wp, T> for Spectrum<T> 
    where T: Float,
        Wp: WhitePoint<T> {

    /// Converts a `Spectrum` to `Xyz` tristimulis values.
    fn into_xyz(self) -> Xyz<Wp, T> {
        let xyz = self.data.into_iter().zip(spectrum_to_xyz_map().iter())
            .fold((T::zero(), T::zero(), T::zero()), |xyz, (intensity, value)| {
                (xyz.0 + *intensity * value.0,
                 xyz.1 + *intensity * value.1,
                 xyz.2 + *intensity * value.2)
            });
        Xyz::with_wp(xyz.0, xyz.1, xyz.2)
    }
}

impl<T: Float + fmt::Debug> fmt::Debug for Spectrum<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

impl<T: Float + PartialEq> PartialEq for Spectrum<T> {
    fn eq(&self, other: &Spectrum<T>) -> bool {
        self.data.iter().eq(other.data.iter())
    }

    fn ne(&self, other: &Spectrum<T>) -> bool {
        !self.eq(other)
    }
}

impl<T: Float> Add<Spectrum<T>> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn add(mut self, other: Spectrum<T>) -> Spectrum<T> {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a = *a + *b;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Add<T> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn add(mut self, c: T) -> Spectrum<T> {
        for sample in self.data.iter_mut() {
            *sample = *sample + c;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Sub<Spectrum<T>> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn sub(mut self, other: Spectrum<T>) -> Spectrum<T> {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a = *a - *b;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Sub<T> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn sub(mut self, c: T) -> Spectrum<T> {
        for sample in self.data.iter_mut() {
            *sample = *sample - c;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Mul<Spectrum<T>> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn mul(mut self, other: Spectrum<T>) -> Spectrum<T> {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a = *a * *b;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Mul<T> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn mul(mut self, c: T) -> Spectrum<T> {
        for sample in self.data.iter_mut() {
            *sample = *sample * c;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Div<Spectrum<T>> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn div(mut self, other: Spectrum<T>) -> Spectrum<T> {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a = *a / *b;
        }
        Spectrum { data: self.data }
    }
}

impl<T: Float> Div<T> for Spectrum<T> {
    type Output = Spectrum<T>;

    fn div(mut self, c: T) -> Spectrum<T> {
        for sample in self.data.iter_mut() {
            *sample = *sample / c;
        }
        Spectrum { data: self.data }
    }
}

#[cfg(test)]
mod test {
    use spectrum::{Sample, Spectrum};
    use spectrum::SAMPLE_COUNT;

    #[test]
    #[should_panic]
    fn test_new_invalid_spectral_data_errors() {
        let data: [f32; SAMPLE_COUNT] = [-1.0; SAMPLE_COUNT];
        Spectrum::new(data);
    }

    #[test]
    fn test_from_samples_empty_slice() {
        let data: &[Sample<f32>] = &[];
        let expected_data: [f32; SAMPLE_COUNT] = [0.0; SAMPLE_COUNT];
        let result = Spectrum::from_samples(data);
        let expected = Spectrum { data: expected_data };
        assert!(
            result == expected,
            format!("{:?} != {:?}", result, expected));
    }

    #[test]
    fn test_from_samples_single_intensity() {
        let data: &[Sample<f32>] = &[Sample::new(360.0, 0.5)];
        let expected_data: [f32; SAMPLE_COUNT] = [0.5; SAMPLE_COUNT];
        let result = Spectrum::from_samples(data);
        let expected = Spectrum { data: expected_data };
        assert!(
            result == expected,
            format!("{:?} != {:?}", result, expected));
    }

    #[test]
    fn test_from_samples_interpolate() {
        let data: &[Sample<f32>] = &[Sample::new(355.0, 0.0), Sample::new(365.0, 1.0)];
        let mut expected_data: [f32; SAMPLE_COUNT] = [1.0; SAMPLE_COUNT];
        expected_data[0] = 0.75;
        let result = Spectrum::from_samples(data);
        let expected = Spectrum { data: expected_data };
        assert!(
            result == expected,
            format!("{:?} != {:?}", result, expected));
    }
}
