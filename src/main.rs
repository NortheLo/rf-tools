use rf_tools::radar_equations::albersheim;
use rf_tools::conversion::db_pwr_to_lin;


fn main() {
    let snr = albersheim(0.5, 1e-6, 1).expect("Should run");
    let some = db_pwr_to_lin(20.0);
    println!("Hello SNR: {}", snr);
}
