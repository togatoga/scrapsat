use core::panic;

use clap::{App, Arg};
use scrapsat::{core::Solver, parser};

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
        .arg(
            Arg::with_name("time")
                .long("time")
                .short("t")
                .takes_value(true)
                .value_name("sec")
                .help("limit on CPU time allowed in seconds"),
        )
        .get_matches();
    let input = matches.value_of("input").expect("input is required");
    let mut solver = Solver::new();
    match parser::parse_cnf(
        std::fs::File::open(input).unwrap_or_else(|_| panic!("can't open file {}", input)),
    ) {
        Ok(cnf) => {
            eprintln!("{:?}", cnf);
            cnf.clauses.iter().for_each(|lits| {
                solver.add_clause(lits);
            });
        }
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("fail to parse");
        }
    }
}
