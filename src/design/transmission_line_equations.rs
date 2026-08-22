use num_traits::Float;

#[derive(Debug)]
pub enum Error {
    InvalidReflectionCoefficient,
    InvalidImpedance,
    NonFinite,
}

#[derive(Debug)]
pub enum Reflection<T: Float> {
    /// Short circuit, corresponding to Γ = -1.
    Short,

    /// Open circuit, corresponding to Γ = +1.
    Open,

    /// General voltage reflection coefficient.
    Coefficient(T),
}

impl<T: Float> Reflection<T> {
    /// Returns the numerical reflection coefficient Γ.
    pub fn coefficient(self) -> T {
        match self {
            Self::Short => -T::one(),
            Self::Open => T::one(),
            Self::Coefficient(gamma) => gamma,
        }
    }
}

/// Calculates the reflection coefficient caused by an impedance mismatch
/// between a reference impedance and a load impedance.
///
/// The reflection coefficient is calculated as:
///
/// `Γ = (Z_L - Z_0) / (Z_L + Z_0)`
///
/// # Arguments
///
/// * `z_0` - Reference impedance in ohms (Ω).
/// * `z_l` - Load impedance in ohms (Ω).
///
/// # Returns
///
/// The voltage reflection coefficient, ranging from -1.0 to 1.0
/// for real-valued impedances.
///
/// Returns [`Error::NonFinite`] if the result is not finite.
///
/// # References
///
/// * [Reflection coefficient](https://en.wikipedia.org/wiki/Reflection_coefficient)
pub fn reflection<T: Float>(z_0: T, z_l: T) -> Result<Reflection<T>, Error> {
    if !z_0.is_finite() || z_0 <= T::zero() {
        return Err(Error::InvalidImpedance);
    }

    if z_l.is_nan() || z_l < T::zero() {
        return Err(Error::InvalidImpedance);
    }

    if z_l == T::zero() {
        return Ok(Reflection::Short);
    }

    if z_l == T::infinity() {
        return Ok(Reflection::Open);
    }

    let gamma = (z_l - z_0) / (z_l + z_0);

    if !gamma.is_finite() {
        return Err(Error::NonFinite);
    }

    Ok(Reflection::Coefficient(gamma))
}

/// Calculates the voltage standing wave ratio (VSWR) from a reflection
/// coefficient.
///
/// VSWR describes the ratio between the maximum and minimum voltage
/// amplitudes on a transmission line:
///
/// `VSWR = (1 + |Γ|) / (1 - |Γ|)`
///
/// # Arguments
///
/// * `reflection_coeff` - Voltage reflection coefficient in the range
///   `-1.0..=1.0`.
///
/// # Returns
///
/// The voltage standing wave ratio (VSWR).
///
/// Returns [`Error::InvalidReflectionCoefficient`] if the reflection
/// coefficient is outside the valid range.
///
/// Returns [`Error::NonFinite`] if the calculated VSWR is not finite.
///
/// # References
///
/// * [Standing wave ratio](https://en.wikipedia.org/wiki/Standing_wave_ratio)
pub fn vswr(reflection_coeff: f64) -> Result<f64, Error> {
    if !reflection_coeff.is_finite() || !(-1.0..=1.0).contains(&reflection_coeff) {
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
        let gamma = reflection(50.0, 50.0).unwrap();

        assert!((gamma.coefficient() - 0.0).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_open_circuit() {
        // Zl -> infinity gives Gamma -> 1
        // Approximate with a very large impedance
        let gamma = reflection(50.0, 1e15).unwrap();

        assert!((gamma.coefficient() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn reflection_coefficient_short_circuit() {
        // Zl = 0 gives Gamma = -1
        let gamma = reflection(50.0, 0.0).unwrap();

        assert!((gamma.coefficient() + 1.0).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_typical_load() {
        // (75 - 50) / (75 + 50) = 0.2
        let gamma = reflection(50.0, 75.0).unwrap();

        assert!((gamma.coefficient() - 0.2).abs() < EPS);
    }

    #[test]
    fn reflection_coefficient_rejects_nan() {
        let result = reflection(f64::NAN, 50.0);

        assert!(result.is_err());
    }

    #[test]
    fn reflection_coefficient_infinity_as_open() {
        let result = reflection(50.0, f64::INFINITY).unwrap();

        assert!(result.coefficient() - 1.0 < EPS);
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
