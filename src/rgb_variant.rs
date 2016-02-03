use num::Float;

use flt;
use Yxy;

pub struct Primaries<T: Float> {
    red: Yxy<T>,
    green: Yxy<T>,
    blue: Yxy<T>,
}

pub trait RgbVariant<T: Float> {
    fn get_primaries() -> Primaries<T>;
}

pub struct AdobeRgbSpace;

impl<T:Float> RgbVariant<T> for AdobeRgbSpace {
    fn get_primaries() -> Primaries<T> {
        Primaries {
            red: Yxy::new(flt(0.6400), flt(0.3300), flt(0.297361)),
            green: Yxy::new(flt(0.2100),	flt(0.7100), flt(0.627355)),
            blue: Yxy::new(flt(0.1500), flt(0.0600),	flt(0.075285)),
        }
    }
}

pub struct SrgbSpace;

impl<T:Float> RgbVariant<T> for SrgbSpace {
    fn get_primaries() -> Primaries<T> {
        Primaries {
            red: Yxy::new(flt(0.6400), flt(0.3300), flt(0.212656)),
            green: Yxy::new(flt(0.3000),	flt(0.6000), flt(0.715158)),
            blue: Yxy::new(flt(0.1500), flt(0.0600),	flt(0.072186)),
        }
    }
}
