use num_traits::Float;
#[derive(Debug)]
pub enum Error {
    InvalidTransmissionPower,
}

/// Link Budget describes the power the signal is received after accounting for all gain and losses
///
/// # Arguments:
///
/// * p_tx: Transmitted power [dBm]
/// * gain_stages: [dB]
/// * losses: [dB]
///
/// # Returns:
///
/// * received signal power [dBm]
pub fn link_budget<T: Float + std::iter::Sum>(
    p_tx: T,
    gain_stages: &[T],
    losses: &[T],
) -> Result<T, Error> {
    if !p_tx.is_finite()
        || gain_stages.iter().any(|x| !x.is_finite())
        || losses.iter().any(|x| !x.is_finite())
    {
        return Err(Error::InvalidTransmissionPower);
    }

    let total_gain: T = gain_stages.iter().copied().sum();
    let total_losses: T = losses.iter().map(|x| x.abs()).sum();

    Ok(p_tx + total_gain - total_losses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_budget() {
        let result = link_budget(20.0_f64, &[10.0, 5.0], &[2.0, 3.0]).unwrap();

        // 20 + 10 + 5 - 2 - 3 = 30
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_link_budget_with_no_gains_or_losses() {
        let result = link_budget(20.0_f64, &[], &[]).unwrap();

        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_link_budget_with_only_gains() {
        let result = link_budget(20.0_f64, &[5.0, 10.0], &[]).unwrap();

        assert_eq!(result, 35.0);
    }

    #[test]
    fn test_link_budget_with_only_losses() {
        let result = link_budget(20.0_f64, &[], &[5.0, 3.0]).unwrap();

        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_link_budget_f32() {
        let result = link_budget(10.0_f32, &[5.0, 2.0], &[1.0]);

        assert_eq!(result.unwrap(), 16.0);
    }
}
