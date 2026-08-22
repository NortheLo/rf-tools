use crate::constants::SPEED_OF_LIGHT;
use num_traits::Float;

pub fn freq_to_wavelength<T: Float>(freq: T) -> T {
    T::from(SPEED_OF_LIGHT).unwrap() / freq
}

pub fn wavelength_to_freq<T: Float>(wavelength: T) -> T {
    T::from(SPEED_OF_LIGHT).unwrap() / wavelength
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-9;

    fn approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {a} ≈ {b}, diff={}",
            (a - b).abs()
        );
    }

    #[test]
    fn test_freq_to_wavelength() {
        // 300 MHz -> ~1 m
        approx_eq(freq_to_wavelength(299_792_458.0), 1.0);
    }

    #[test]
    fn test_wavelength_to_freq() {
        // 1 m -> ~300 MHz
        approx_eq(wavelength_to_freq(1.0), 299_792_458.0);
    }
}
