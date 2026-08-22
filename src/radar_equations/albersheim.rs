use super::radar_equations_errors::RadarEquationsErrors;

pub fn albersheim(p_d: f64, p_fa: f64, n: u64) -> Result<f64, RadarEquationsErrors> {
    // Albersheim equation for calculating the SNR [dB] of non-fluctuation targets based on
    // P_d: Detection probability
    // P_fa: False-Alarm probability
    // N: Number of noncoherent integrated pulses
    // Returns: SNR
    // Source: https://radarsp.weebly.com/uploads/2/1/4/7/21471216/albersheim_alternative_forms.pdf
    if 0.0 >= p_d && p_d > 1.0 {
        return Err(RadarEquationsErrors::InvalidDetectionProbability);
    }

    if 0.0 >= p_fa && p_fa > 1.0 {
        return Err(RadarEquationsErrors::InvalidFalseAlarmProbability);
    }

    if n == 0 {
        return Err(RadarEquationsErrors::InvalidPulseCount);
    }

    let a = (0.62 / p_fa).ln();
    let b = (p_d / (1.0 - p_d)).ln();

    let snr = -5.0 * (n as f64).log10()
            + (6.2 + 4.54 / (n as f64 + 0.44).sqrt())
            * (a + 0.12 * a * b + 1.7 * b).log10();

    if snr.is_finite() {
        Ok(snr)
    }
    else {
        Err(RadarEquationsErrors::IsInvalid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const EPS: f64 = 1e-9;

    #[test]
    fn test_albersheim() {
        // Results are from MATLAB albersheim function
        let snr = albersheim(0.5, 1e-6, 1).unwrap();
        let expected = 11.231984877880109;
        assert!((snr - expected).abs() < EPS);

        let snr = albersheim(0.9, 1e-6, 10).unwrap();
        let expected = 4.990385959428693;
        assert!((snr - expected).abs() < EPS);
    }


    proptest! {
        #[test]
        fn finite_result(
            // Not every combination results in a real fp solution
            // MATLAB just errors out
            p_d in 0.001f64..0.999,
            p_fa in 1e-12f64..0.1,
            n in 1u64..1000,
        ) {
            let snr_res = albersheim(p_d, p_fa, n);

            if let Ok(snr) = snr_res {
                prop_assert!(snr.is_finite());
            }
        }
    }
}
