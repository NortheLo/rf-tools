use crate::conversion::lin_to_db_amp;
use num_traits::Float;
pub fn friis_equation_lin<T: Float>(p_tx: T, g_tx: T, g_rx: T, lambda: T, distance: T) -> T {
    let four_pi = T::from(4.0 * std::f64::consts::PI).unwrap();

    let factor = lambda / (four_pi * distance);
    p_tx * g_tx * g_rx * factor.powi(2)
}

pub fn friis_equation_log<T: Float>(p_tx: T, g_tx: T, g_rx: T, lambda: T, distance: T) -> T {
    let four_pi = T::from(4.0 * std::f64::consts::PI).unwrap();

    p_tx + g_tx + g_rx + lin_to_db_amp(lambda / (four_pi * distance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::*;

    // Reference Values can be generated here: https://electrotopic.com/de/friis-ubertragungsrechner/#gsc.tab=0
    // Settings are:
    // Pt = 10 Watts
    // Gt = 3
    // Gr = 3
    // lambda = 0.5
    // R = 2

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {a} ≈ {b}, got diff={}",
            (a - b).abs()
        );
    }

    #[test]
    fn test_friis_transmission_linear() {
        let p_tx = 10.0;
        let g_tx = 2.0;
        let g_rx = 2.0;
        let lambda = 0.5;
        let distance = 2.0;

        let result = friis_equation_lin(p_tx, g_tx, g_rx, lambda, distance);

        let expected = 0.015831434944115281;
        approx_eq(result, expected);
    }

    #[test]
    fn test_friis_transmission_log() {
        let p_tx = 10.0;
        let g_tx = 2.0;
        let g_rx = 2.0;
        let lambda = 0.5;
        let distance = 2.0;

        let result = friis_equation_log(
            w_to_dbm(p_tx),
            lin_to_db_pwr(g_tx),
            lin_to_db_pwr(g_rx),
            lambda,
            distance,
        );

        let expected = 11.995202806278449;
        approx_eq(result, expected);
    }
}
