mod config;
mod scan;

use std::env;
use config::Config;
use scan::{ScanTableSt, scan_table_scan};

fn main() {

    let config = Config::parse_args(env::args().collect());

    let mut st = ScanTableSt::new();
    scan_table_scan(&mut st, config.expression.as_bytes());
    st.print();

    println!("Expression: {}", config.expression);
}
