mod config;
mod scan;
mod parse;
mod eval;

use std::env;
use config::Config;
use scan::ScanTableSt;
use parse::ParseTable;

fn main() {

    let config = Config::parse_args(env::args().collect());

    let mut scan_table = ScanTableSt::new();
    scan_table.scan(&config.expression);

    let mut parse_table = ParseTable::new();
    parse_table.parse(&mut scan_table);


    let config = Config::parse_args(env::args().collect());

}


