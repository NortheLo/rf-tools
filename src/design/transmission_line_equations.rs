#[derive(Debug)]
enum Error {
    InvalidReflectionCoefficient,
    NonFinite,
}
fn reflection_coefficient(z_0: f64, z_l: f64) -> Result<f64, Error> {
    // Reflection coefficient based on the impedance mismatch between source and load.
    // Args:
    // Z_0: Reference Impedance
    // Z_L: Load Impedance
    // Returns:
    // Reflection Coefficient
    // Source: https://en.wikipedia.org/wiki/Reflection_coefficient
    let ref_coef = (z_l - z_0) / (z_l + z_0);

    ref_coef
        .is_finite()
        .then_some(ref_coef)
        .ok_or(Error::NonFinite)
}

fn vswr(reflection_coeff: f64) -> Result<f64, Error> {
    // Voltage-Standing-Wave-Ratio defines the relationship between |V_max| / |V_min|
    // Args:
    // Reflection Coefficient
    // Returns:
    // VSWR
    // Source: https://en.wikipedia.org/wiki/Standing_wave_ratio
    if reflection_coeff > 1.0 || reflection_coeff < -1.0 {
        return Err(Error::InvalidReflectionCoefficient);
    }

    let vswr = (1.0 + reflection_coeff.abs()) / (1.0 - reflection_coeff.abs());

    vswr.is_finite().then_some(vswr).ok_or(Error::NonFinite)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    #[test]
    fn reflection_coefficient_matched_load() {
        // Zl == Z0 -> no reflection
        let gamma = reflection_coefficient(50.0, 50.0).unwrap();

        assert!((gamma - 0.0).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_open_circuit() {
        // Zl -> infinity gives Gamma -> 1
        // Approximate with a very large impedance
        let gamma = reflection_coefficient(50.0, 1e15).unwrap();

        assert!((gamma - 1.0).abs() < 1e-12);
    }

    #[test]
    fn reflection_coefficient_short_circuit() {
        // Zl = 0 gives Gamma = -1
        let gamma = reflection_coefficient(50.0, 0.0).unwrap();

        assert!((gamma + 1.0).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_typical_load() {
        // (75 - 50) / (75 + 50) = 0.2
        let gamma = reflection_coefficient(50.0, 75.0).unwrap();

        assert!((gamma - 0.2).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_rejects_nan() {
        let result = reflection_coefficient(f64::NAN, 50.0);

        assert!(result.is_err());
    }

    #[test]
    fn reflection_coefficient_rejects_infinity() {
        let result = reflection_coefficient(50.0, f64::INFINITY);

        assert!(result.is_err());
    }

    #[test]
    fn vswr_zero_reflection() {
        let result = vswr(0.0).unwrap();

        assert!((result - 1.0).abs() < EPS);
    }

    #[test]
    fn vswr_positive_reflection() {
        // Gamma = 0.5 -> VSWR = 3
        let result = vswr(0.5).unwrap();

        assert!((result - 3.0).abs() < EPS);
    }

    #[test]
    fn vswr_negative_reflection() {
        // VSWR depends on magnitude only
        let result = vswr(-0.5).unwrap();

        assert!((result - 3.0).abs() < EPS);
    }

    #[test]
    fn vswr_rejects_gamma_greater_than_one() {
        let result = vswr(1.1);

        assert!(result.is_err());
    }

    #[test]
    fn vswr_rejects_nan() {
        let result = vswr(f64::NAN);

        assert!(result.is_err());
    }

    #[test]
    fn vswr_rejects_infinity() {
        let result = vswr(f64::INFINITY);

        assert!(result.is_err());
    }
}
