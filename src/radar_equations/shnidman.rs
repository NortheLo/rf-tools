use num_traits::signum;
use num_traits::float::Float;
use super::radar_equations_errors::RadarEquationsErrors;

#[derive(Debug, Clone, Copy)]
pub enum SwerlingCase {
    I,
    II,
    III,
    IV,
    V,
}

impl SwerlingCase {
    pub fn k(&self, n: usize) -> f64 {
        match self {
            SwerlingCase::I => 1.0,

            SwerlingCase::II => n as f64,

            SwerlingCase::III => 2.0,

            SwerlingCase::IV => 2.0 * n as f64,

            SwerlingCase::V => f64::INFINITY,
        }
    }

    pub fn alpha(&self, n: usize) -> f64 {
        match self {
            _ => {
                if n < 40 {
                    0.0
                } else {
                    0.25
                }
            }
        }
    }
}

pub fn shnidman(p_d: f64, p_fa: f64, n: usize, model: SwerlingCase) -> Result<f64, RadarEquationsErrors> {
    // shnidman equation for fluctuating targets
    // Args:
    // P_d: Probability of detection
    // P_fa: Fals-Alarm Probability
    // n: Number in incoherent integrated pulses
    // Returns:
    // SNR: Minimum SNR needed
    // Source:
    // "Radar Detection Probabilities and Their Calculation" 1995 David A. Shnidmann
    // "Fundamentals of Radar Signal Processing" Mark A. Richards
    if 0.0 >= p_d && p_d > 1.0 {
        return Err(RadarEquationsErrors::InvalidDetectionProbability);
    }

    if 0.0 >= p_fa && p_fa > 1.0 {
        return Err(RadarEquationsErrors::InvalidFalseAlarmProbability);
    }

    if n == 0 {
        return Err(RadarEquationsErrors::InvalidPulseCount);
    }

    let k = model.k(n);
    let alpha = model.alpha(n);

    let eta = Float::sqrt(-0.8 * Float::ln(4.0 * p_fa * (1.0 - p_fa))) +
                    signum(p_d - 0.5) * Float::sqrt(-0.8 * Float::ln(4.0 * p_d * (1.0 - p_d)));
    let x_inf = eta * (eta + 2.0 * Float::sqrt(n as f64 / 2.0 + (alpha - 0.25)));

    let c_1 = (((17.7006 * p_d - 18.4496) * p_d + 14.5339) * p_d - 3.525) / k;
    let c_2 = (1.0 / k) * (Float::exp(27.31 * p_d - 25.14) +
                    (p_d - 0.8) * (0.7 * Float::ln(1e-5 / p_fa) +
                        (2.0 * n as f64 - 20.0)/80.0));

    let c_db = if 0.1 <= p_d && p_d <= 0.872 { c_1 } else { c_1 + c_2 };
    let c = Float::powf(10.0, c_db/10.0);

    let x = 10.0 * Float::log10(c * x_inf / n as f64);

    x.is_finite().then_some(x).ok_or(RadarEquationsErrors::IsInvalid)
}