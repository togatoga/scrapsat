use regex::Regex;

use crate::lit::Lit;
use std::io::BufRead;
/// CnfData represents parsed data
#[derive(Debug)]
pub struct CnfData {
    // the number of variable
    pub num_variable: Option<u32>,
    // the number of clause
    pub num_clause: Option<u32>,
    // all problem clauses
    pub clauses: Vec<Vec<Lit>>,
}
/// Parse a DIMACAS cnf file
/// # Arguments
/// * `input_file` - A path of an input file name
/// c Here is a comment.
/// c SATISFIABLE
/// p cnf 5 3
/// 1 -5 4 0
/// -1 5 3 4 0
/// -3 -4 0
pub fn parse_cnf<R: std::io::Read>(input: R) -> std::io::Result<CnfData> {
    let reader = std::io::BufReader::new(input);
    let mut num_variable = None;
    let mut num_clause = None;
    let mut clauses = vec![];

    let re = Regex::new(r"p cnf (\d+) (\d+)").expect("bad regex");
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if num_variable.is_none() && num_clause.is_none() {
            // p cnf 5 3
            if let Some(caps) = re.captures(line) {
                let var = caps.get(1).expect("fail to parse").as_str().parse::<u32>();
                let clause = caps.get(2).expect("fail to parse").as_str().parse::<u32>();
                if var.is_ok() && clause.is_ok() {
                    num_variable = Some(var.expect("bad num variable"));
                    num_clause = Some(clause.expect("bad num clause"));
                }
            }
        }

        let values: Vec<&str> = line.split_whitespace().collect();

        if values.is_empty() || values[0] == "c" {
            // empty or comment
            continue;
        }

        let values: Vec<_> = values
            .into_iter()
            .filter_map(|x| x.parse::<i32>().ok())
            .take_while(|x| *x != 0)
            .collect();

        if values.is_empty() {
            // skip an empty line
            continue;
        }
        let clause: Vec<Lit> = values.iter().map(|&x| Lit::from(x)).collect();
        clauses.push(clause);
    }
    Ok(CnfData {
        num_variable,
        num_clause,
        clauses,
    })
}
