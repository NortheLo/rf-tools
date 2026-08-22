use crate::constants::BOLTZMANN_CONSTANT;
use num_traits::{Float, FromPrimitive};

#[derive(Debug)]
pub enum Error {
    NonFinite,
    InvalidBandwidth,
    InvalidTemperature,
}

/// Power of thermal noise in defined by P_noise = k_b * T * BW
///
/// # Arguments:
///
/// Temperature [K]
/// Bandwidth [1/s]
///
/// # Returns:
///
/// Power of noise [lin]
///
/// Source: https://en.wikipedia.org/wiki/Johnson%E2%80%93Nyquist_noise
pub fn thermal_noise_power<T: Float + FromPrimitive>(
    temperature: T,
    bandwidth: T,
) -> Result<T, Error> {
    if !temperature.is_finite() || temperature < T::zero() {
        return Err(Error::InvalidTemperature);
    }

    if !bandwidth.is_finite() || bandwidth < T::zero() {
        return Err(Error::InvalidBandwidth);
    }

    let k = T::from_f64(BOLTZMANN_CONSTANT)
        .expect("Boltzmann constant must be representable as a floating-point value");

    let np = k * temperature * bandwidth;

    np.is_finite().then_some(np).ok_or(Error::NonFinite)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-30;

    #[test]
    fn thermal_noise_power_basic() {
        // 290 K receiver temperature, 1 MHz bandwidth
        // P = k * T * BW
        let power = thermal_noise_power(290.0, 1e6).unwrap();

        let expected = BOLTZMANN_CONSTANT * 290.0 * 1e6;

        assert!((power - expected).abs() < EPS);
    }

    #[test]
    fn thermal_noise_power_zero_bandwidth() {
        let power = thermal_noise_power(300.0, 0.0).unwrap();

        assert_eq!(power, 0.0);
    }

    #[test]
    fn thermal_noise_power_rejects_nan() {
        let result = thermal_noise_power(f64::NAN, 1e6);

        assert!(result.is_err());
    }

    #[test]
    fn thermal_noise_power_rejects_infinity() {
        let result = thermal_noise_power(300.0, f64::INFINITY);

        assert!(result.is_err());
    }
}
