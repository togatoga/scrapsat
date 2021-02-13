use core::panic;

use clap::{App, Arg};
use scrapsat::{core::Solver, parser};
use signal_hook::{consts::SIGINT, iterator::Signals};

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
            cnf.clauses.iter().for_each(|lits| {
                solver.add_clause(lits);
            });

            let sender = solver.sender.clone();
            let mut signals = Signals::new(&[SIGINT]).expect("togatoga");
            std::thread::spawn(move || {
                for sig in signals.forever() {
                    eprintln!("{:?}", sig);
                    sender.send(0).expect("failed to send");
                }
            });

            match solver.solve() {
                scrapsat::core::SatResult::Sat => {
                    println!("c SAT");
                }
                scrapsat::core::SatResult::Unsat => {
                    println!("c UNSAT");
                }
                scrapsat::core::SatResult::Unknown => {
                    println!("c UNKNOWN");
                }
            };
        }
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("failed to parse");
        }
    }
}
