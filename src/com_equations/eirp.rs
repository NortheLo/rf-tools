use num_traits::Float;

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
pub fn eirp<T: Float>(p_tx: T, antenna_gain: T) -> T {
    p_tx + antenna_gain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eirp() {
        let result = eirp(20.0_f64, 5.0_f64);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_eirp_with_zero_gain() {
        let result = eirp(20.0_f64, 0.0_f64);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_eirp_with_negative_gain() {
        let result = eirp(20.0_f64, -3.0_f64);
        assert_eq!(result, 17.0);
    }
}
