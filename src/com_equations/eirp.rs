use num_traits::Float;

#[derive(Debug)]
pub enum Error {
    NonFiniteTransmissionPower,
    NonFiniteAntennaGain,
}
/// Equivalent Isotropically Radiated Power (EIRP) is the radiated power from
/// an isotropic antenna.
///
/// # Arguments:
///
/// * p_tx: Transmitted Power in log
/// * antenna_gain: Gain of the antenna over an hypothetical lossless half-wave dipole
///
/// # References:
/// https://en.wikipedia.org/wiki/Effective_radiated_power
pub fn eirp<T: Float>(p_tx: T, antenna_gain: T) -> Result<T, Error> {
    if !p_tx.is_finite() {
        return Err(Error::NonFiniteTransmissionPower)
    }
    if !antenna_gain.is_finite() {
        return Err(Error::NonFiniteAntennaGain)
    }

    Ok(p_tx + antenna_gain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eirp() {
        let result = eirp(20.0_f64, 5.0_f64).unwrap();
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_eirp_with_zero_gain() {
        let result = eirp(20.0_f64, 0.0_f64).unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_eirp_with_negative_gain() {
        let result = eirp(20.0_f64, -3.0_f64).unwrap();
        assert_eq!(result, 17.0);
    }

    #[test]
    fn test_non_finite_p_tx() {
        let result = eirp(f64::INFINITY, 1.0);

        assert!(result.is_err());
    }

    #[test]
    fn test_non_finite_antenna_gain() {
        let result = eirp(1.0, f64::INFINITY);

        assert!(result.is_err());
    }
}
