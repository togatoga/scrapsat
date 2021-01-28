use core::panic;

use clap::{App, Arg};
use scrapsat::parser;

fn main() {
    let matches = App::new("scrapsat")
        .version("0.1")
        .author("Hitoshi Togasaki <togasakitogatoga+github@gmail.com")
        .about("SAT solver")
        .arg(
            Arg::with_name("input")
                .help("input CNF file")
                .index(1)
                .required(true),
        )
        .get_matches();
    let input = matches.value_of("input").expect("input is required");
    match parser::parse_cnf(
        std::fs::File::open(input).unwrap_or_else(|_| panic!("can't open file {}", input)),
    ) {
        Ok(cnf) => {
            println!("{:?} {:?}", cnf.num_variable, cnf.num_clause);
            eprintln!("{:?}", cnf);
        }
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("fail to parse");
        }
    }
}
