use rf_tools::conversion::power::db_pwr_to_lin;
use rf_tools::radar_equations::{SwerlingCase, albersheim, shnidman};

fn main() {
    let snr = albersheim(0.5, 1e-6, 1).expect("Should run");
    let _some = db_pwr_to_lin(20.0);
    println!("Hello SNR: {}", snr);

    let snr_shidman = shnidman(0.5, 1e-6, 1, SwerlingCase::I).expect("Should run");
    println!("Hello Shnidman: {}", snr_shidman);
}
