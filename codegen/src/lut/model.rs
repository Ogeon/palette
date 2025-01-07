use super::TransferFn;

/// This struct contains the scale and bias for a linear
/// regression model of a transfer function on a given interval.
///
/// This model is calculated by using simple linear regression with
/// integration instead of summation.
pub(super) struct LinearModel {
    scale: f64,
    bias: f64,
}

impl LinearModel {
    pub(super) fn new(
        transfer_fn: &TransferFn,
        start: u32,
        end: u32,
        man_index_width: u32,
        t_width: u32,
    ) -> Self {
        let TransferFn {
            linear_scale,
            alpha,
            beta,
            gamma,
            ..
        } = *transfer_fn;

        let beta_bits = (beta as f32).to_bits();
        // Corresponds to the scale between differentials. Specifically,
        // `dx = exp_scale * dt`
        let exp_scale = f32::from_bits(((start >> 23) - man_index_width - t_width) << 23) as f64;
        let start_x = f32::from_bits(start) as f64;
        let end_x = f32::from_bits(end) as f64;

        // If the transfer function is purely linear on a given interval,
        // integration is unnecessary.
        if let Some(linear_scale) = linear_scale {
            if end <= beta_bits {
                return Self {
                    scale: linear_scale * exp_scale,
                    bias: linear_scale * start_x,
                };
            }
        }

        let max_t = 2.0f64.powi(t_width as i32);

        let (integral_y, integral_ty) = match linear_scale {
            Some(linear_scale) if start < beta_bits => {
                let beta_t =
                    (beta_bits << (9 + man_index_width)) as f64 * 2.0f64.powi(t_width as i32 - 32);
                let int_linear =
                    integrate_linear((start_x, beta), (0.0, beta_t), linear_scale, exp_scale);
                let int_exponential =
                    integrate_exponential((beta, end_x), (beta_t, max_t), alpha, gamma, exp_scale);
                (
                    int_linear.0 + int_exponential.0,
                    int_linear.1 + int_exponential.1,
                )
            }
            _ => integrate_exponential((start_x, end_x), (0.0, max_t), alpha, gamma, exp_scale),
        };
        let max_t2 = max_t * max_t;
        let integral_t = max_t2 * 0.5;
        let integral_t2 = max_t2 * max_t / 3.0;

        let scale = (max_t * integral_ty - integral_t * integral_y)
            / (max_t * integral_t2 - integral_t * integral_t);
        Self {
            scale,
            bias: (integral_y - scale * integral_t) / max_t,
        }
    }

    pub(super) fn into_u8_lookup(self) -> u32 {
        let scale_uint = (255.0 * self.scale * 65536.0 + 0.5) as u32;
        let bias_uint = (((255.0 * self.bias + 0.5) * 128.0 + 0.5) as u32) << 9;
        (bias_uint << 7) | scale_uint
    }

    pub(super) fn into_u16_lookup(self) -> u64 {
        let scale_uint = (65535.0 * self.scale * 4294967296.0 + 0.5) as u64;
        let bias_uint = (((65535.0 * self.bias + 0.5) * 32768.0 + 0.5) as u64) << 17;
        (bias_uint << 15) | scale_uint
    }
}

fn integrate_linear(
    (start_x, end_x): (f64, f64),
    (start_t, end_t): (f64, f64),
    linear_scale: f64,
    exp_scale: f64,
) -> (f64, f64) {
    let antiderive_y = |x: f64| 0.5 * linear_scale * x * x / exp_scale;
    let antiderive_ty =
        |x: f64, t: f64| 0.5 * linear_scale * x * x * (t - x / (3.0 * exp_scale)) / exp_scale;

    (
        antiderive_y(end_x) - antiderive_y(start_x),
        antiderive_ty(end_x, end_t) - antiderive_ty(start_x, start_t),
    )
}

fn integrate_exponential(
    (start_x, end_x): (f64, f64),
    (start_t, end_t): (f64, f64),
    alpha: f64,
    gamma: f64,
    exp_scale: f64,
) -> (f64, f64) {
    let one_plus_gamma_inv = 1.0 + gamma.recip();
    let antiderive_y = |x: f64, t: f64| {
        alpha * gamma * x.powf(one_plus_gamma_inv) / (exp_scale * (1.0 + gamma)) + (1.0 - alpha) * t
    };
    let antiderive_ty = |x: f64, t: f64| {
        alpha
            * gamma
            * x.powf(one_plus_gamma_inv)
            * (t - gamma * x / (exp_scale * (1.0 + 2.0 * gamma)))
            / (exp_scale * (1.0 + gamma))
            + 0.5 * (1.0 - alpha) * t * t
    };

    (
        antiderive_y(end_x, end_t) - antiderive_y(start_x, start_t),
        antiderive_ty(end_x, end_t) - antiderive_ty(start_x, start_t),
    )
}
