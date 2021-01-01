fn main() {
    let matches = clap::App::new("scrapsat")
        .version("0.1.0")
        .author("Hitoshi Togasaki")
        .about("SAT Solver")
        .arg(
            clap::Arg::with_name("input")
                .help("An input CNF file")
                .required(true)
                .index(1),
        )
        .get_matches();
    let input_cnf_file = std::fs::File::open(matches.value_of("input").unwrap()).unwrap();
    let mut solver = scrapsat::solver::Solver::new();

    if let Err(e) = scrapsat::dimacs::parse_dimacs_file(&input_cnf_file, &mut solver, None) {
        println!("{}", e);
    } else {
        match solver.solve_limited() {
            scrapsat::lit::LitBool::True => {
                eprintln!("SAT");
            },
            scrapsat::lit::LitBool::False => {
                eprintln!("UNSAT");
            },
            scrapsat::lit::LitBool::Undef => {
                eprintln!("Undef");
            }
        }
        
    }
}
