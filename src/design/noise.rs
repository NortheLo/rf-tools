use crate::constants::BOLTZMANN_CONSTANT;

#[derive(Debug)]
pub enum Error {
    NonFinite,
    InvalidBandwidth,
    InvalidTemperature,
}

pub fn thermal_noise_power(temperature: f64, bandwidth: f64) -> Result<f64, Error> {
    // Power of thermal noise in defined by P_noise = k_b * T * BW
    // Args:
    // Temperature [K]
    // Bandwidth [1/s]
    // Source: https://en.wikipedia.org/wiki/Johnson%E2%80%93Nyquist_noise
    if !temperature.is_finite() || temperature < 0.0 {
        return Err(Error::InvalidTemperature);
    }

    if !bandwidth.is_finite() || bandwidth < 0.0 {
        return Err(Error::InvalidBandwidth);
    }

    let np = BOLTZMANN_CONSTANT * temperature * bandwidth;

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
