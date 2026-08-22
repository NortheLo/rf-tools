use num_traits::Float;

pub fn lin_to_db_pwr<T: Float>(x: T) -> T {
    T::from(10.0).unwrap() * x.log10()
}

pub fn lin_to_db_amp<T: Float>(x: T) -> T {
    T::from(2.0).unwrap() * lin_to_db_pwr(x)
}

pub fn db_pwr_to_lin<T: Float>(x: T) -> T {
    T::from(10.0).unwrap().powf(x / T::from(10.0).unwrap())
}

pub fn db_amp_to_lin<T: Float>(x: T) -> T {
    db_pwr_to_lin(x / T::from(2.0).unwrap())
}

pub fn w_to_dbm<T: Float>(p: T) -> T {
    lin_to_db_pwr(p) + T::from(30.0).unwrap()
}

pub fn dbm_to_w<T: Float>(p: T) -> T {
    db_pwr_to_lin(p) / T::from(1000.0).unwrap()
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
    fn test_lin_to_db_pwr() {
        approx_eq(lin_to_db_pwr(100.0), 20.0);
    }

    #[test]
    fn test_lin_to_db_amp() {
        approx_eq(lin_to_db_amp(10.0), 20.0);
    }

    #[test]
    fn test_db_pwr_to_lin() {
        approx_eq(db_pwr_to_lin(20.0), 100.0);
    }

    #[test]
    fn test_db_amp_to_lin() {
        approx_eq(db_amp_to_lin(20.0), 10.0);
    }

    #[test]
    fn test_w_to_dbm() {
        // 1 W = 30 dBm
        approx_eq(w_to_dbm(1.0), 30.0);
    }

    #[test]
    fn test_dbm_to_w() {
        // 30 dBm = 1 W
        approx_eq(dbm_to_w(30.0), 1.0);
    }
}
