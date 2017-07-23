use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use errors::*;
use flt;
use num::{Float, FromPrimitive, Zero};
use ordered_float::OrderedFloat;
use white_point::WhitePoint;
use xyz::Xyz;

/// The smallest wavelength represented in the `Spectrum`
/// data structure. This is the first value in the
/// map's wavelength.
pub const MIN_LAMBDA: usize = 360;

/// The largest wavelength represented in the `Spectrum`
/// data structure. This is the last value in the map's
/// wavelength.
#[allow(dead_code)]
pub const MAX_LAMBDA: usize = 830;

const SAMPLE_STEP: usize = 5;

pub const N_SAMPLES: usize = (MAX_LAMBDA - MIN_LAMBDA) / SAMPLE_STEP + 1;

/// Lookup table for converting a wavelength in the range
/// [360, 830] nm into XYZ tristimulus values.
fn spectrum_to_xyz_map<T: Float>() -> [(T, T, T); N_SAMPLES] {
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

/// Spectrum representing the relative intensity of 
/// wavelengths from 360 nm to 830 nm.
pub struct Spectrum<T: Float> {
    data: [T; N_SAMPLES],
}

impl<T: Float> Clone for Spectrum<T> {
    fn clone(&self) -> Spectrum<T> {
        let mut data: [T; N_SAMPLES] = [T::zero(); N_SAMPLES];
        data.copy_from_slice(&self.data);
        Spectrum {
            data: data,
        }
    }
}

impl<T> Spectrum<T>
    where T: Float + Zero + FromPrimitive
{
    /// Create a Spectrum from an array of spectral intensity
    /// values assumed to be in the range 360 nm to 830 nm.
    pub fn new(data: [T; N_SAMPLES]) -> Result<Self> {
        if data.iter().any(|&intensity| intensity < T::zero()) {
            Err(ErrorKind::SpectrumIntensityOutOfRange.into())
        } else {
            Ok(Spectrum { data: data })
        }
    }

    /// Create a `Spectrum` from a slice of (f32, Float) tuples
    /// representing a (wavelength, intensity) mapping.
    /// 
    /// The more data points there are the more accurate the 
    /// `Spectrum`'s internal representation will be.
    pub fn from_sparse(data: &[(f32, T)]) -> Result<Self> {
        // TODO: replace this with sort_unstable_by() when stabilised.
        let mut data: Vec<(OrderedFloat<f32>, T)> = data
            .iter()
            .map(|&(l, i)| (OrderedFloat(l), i))
            .collect();
        data.sort_by(|a, b| a.0.cmp(&b.0));
        let data: Vec<(f32, T)> = data
            .into_iter()
            .map(|(l, i)| (l.into_inner(), i))
            .collect();
        if data.len() == 0 {
            return Ok(Spectrum { data: [T::zero(); N_SAMPLES] });
        }
        if data.iter().any(|&(_, intensity)| intensity < T::zero()) {
            return Err(ErrorKind::SpectrumIntensityOutOfRange.into());
        }
        // If there is only one data point we just have a constant
        // intensity for the entire spectrum.
        if data.len() == 1 {
            return Ok(Spectrum { data: [data[0].1; N_SAMPLES] });
        }
        let mut sampled: [T; N_SAMPLES] = [T::zero(); N_SAMPLES];
        for (lambda, sample) in (0..N_SAMPLES)
            .map(|i| (MIN_LAMBDA + (i * SAMPLE_STEP)) as f32)
            .zip(sampled.iter_mut())
        {
            if lambda >= data.last().unwrap().0 {
                *sample = data.last().unwrap().1;
            } else if lambda <= data.first().unwrap().0 {
                *sample = data.first().unwrap().1;
            } else {
                // Find the upper and lower wavelengths bounding `lambda`
                // and interpolate between them to find the intensity.
                for patch in data.windows(2) {
                    let (lo_lambda, lo_intensity) = patch[0];
                    let (hi_lambda, hi_intensity) = patch[1];
                    if lambda >= lo_lambda && lambda < hi_lambda {
                        let t = T::from_f32((lambda - lo_lambda) / (hi_lambda - lo_lambda))
                            .expect("Failed to convert f32 to Float.");
                        *sample = lerp(lo_intensity, hi_intensity, t);
                        break;
                    }
                }
            }
        }
        Ok(Spectrum { data: sampled })
    }

    /// Converts a `Spectrum` to `Xyz` tristimulis values.
    pub fn to_xyz<Wp: WhitePoint<T>>(&self) -> Xyz<Wp, T> {
        let xyz = self.data.iter().zip(spectrum_to_xyz_map().iter())
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
    use spectrum::Spectrum;
    use spectrum::N_SAMPLES;

    #[test]
    fn test_new_invalid_spectral_data_errors() {
        let data: [f32; N_SAMPLES] = [-1.0; N_SAMPLES];
        let result = Spectrum::new(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_sparse_empty_slice() {
        let data: &[(f32, f32)] = &[];
        let expected_data: [f32; N_SAMPLES] = [0.0; N_SAMPLES];
        let result = Spectrum::from_sparse(data);
        assert!(result.is_ok());
        assert!(result.unwrap() == Spectrum { data: expected_data });
    }

    #[test]
    fn test_sparse_single_intensity() {
        let data: &[(f32, f32)] = &[(360.0, 0.5)];
        let expected_data: [f32; N_SAMPLES] = [0.5; N_SAMPLES];
        let result = Spectrum::from_sparse(data);
        assert!(result.is_ok());
        assert!(result.unwrap() == Spectrum { data: expected_data });
    }

    #[test]
    fn test_sparse_interpolate() {
        let data: &[(f32, f32)] = &[(355.0, 0.0), (365.0, 1.0)];
        let mut expected_data: [f32; N_SAMPLES] = [1.0; N_SAMPLES];
        // The first value (360) will be interpolated between 355 and 365.
        expected_data[0] = 0.5;
        let result = Spectrum::from_sparse(data);
        assert!(result.is_ok());
        assert!(result.unwrap() == Spectrum { data: expected_data });
    }
}
