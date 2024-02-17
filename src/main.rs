mod config;
mod scan;
mod parse;
mod eval;
mod test;
use std::env;
use config::Config;
use scan::ScanTableSt;
use parse::{ParseTableSt};
use eval::{eval};
fn main() {

    let config = Config::parse_args(env::args().collect());

    let mut scan_table = ScanTableSt::new();
    scan_table.scan(&config.expression);

    let mut parse_table = ParseTableSt::new();
    let mut parse_node = parse_table.parse_program(&mut scan_table);

    let value = eval(&parse_node.as_ref());
    println!("{}", value);


}


