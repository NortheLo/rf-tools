mod radar_equations;
mod conversion;
pub mod telcom_equations;

use crate::radar_equations::albersheim;
use conversion::db_pwr_to_lin;


fn main() {
    let snr = albersheim(0.5, 1e-6, 1).expect("Should run");
    let some = db_pwr_to_lin(20.0);
    println!("Hello SNR: {}", snr);
}
