use crate::constants::BOLTZMANN_CONSTANT;

#[derive(Debug)]
enum Error {
    NonFinite,
}

fn thermal_noise_power(temperature: f64, bandwidth: f64) -> Result<f64, Error> {
    // Power of thermal noise in defined by P_noise = k_b * T * BW
    // Args:
    // Temperature [K]
    // Bandwidth [1/s]
    // Source: https://en.wikipedia.org/wiki/Johnson%E2%80%93Nyquist_noise
    let np = BOLTZMANN_CONSTANT * temperature * bandwidth;

    np.is_finite().then_some(np).ok_or(Error::NonFinite)

}