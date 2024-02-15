mod config;
mod scan;
mod parse;
mod ntlang;

use std::env;
use config::Config;
use scan::{ScanTableSt, scan_table_scan};

fn main() {

    let config = Config::parse_args(env::args().collect());

    let mut st = ScanTableSt::new();
    scan_table_scan(&mut st, config.expression.as_bytes());
    st.print();

    parse_table_init(&parse_table);
    parse_tree = parse_program(&parse_table, &scan_table);

    println!("Expression: {}", config.expression);
}
