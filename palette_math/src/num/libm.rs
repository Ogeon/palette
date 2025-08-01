use super::*;

impl Powf for f32 {
    #[inline]
    fn powf(self, exp: Self) -> Self {
        ::libm::powf(self, exp)
    }
}

impl Powf for f64 {
    #[inline]
    fn powf(self, exp: Self) -> Self {
        ::libm::pow(self, exp)
    }
}

impl Powi for f32 {
    #[inline]
    fn powi(mut self, mut exp: i32) -> Self {
        if exp < 0 {
            exp = exp.wrapping_neg();
            self = self.recip();
        }

        Powu::powu(self, exp as u32)
    }
}

impl Powi for f64 {
    #[inline]
    fn powi(mut self, mut exp: i32) -> Self {
        if exp < 0 {
            exp = exp.wrapping_neg();
            self = self.recip();
        }

        Powu::powu(self, exp as u32)
    }
}

impl Recip for f32 {
    #[inline]
    fn recip(self) -> Self {
        1.0 / self
    }
}

impl Recip for f64 {
    #[inline]
    fn recip(self) -> Self {
        1.0 / self
    }
}
