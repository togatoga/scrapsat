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

    for line in reader.lines() {
        let line = line?;
        // trim extra/duplicate whitespaces
        let values: Vec<_> = line.split_whitespace().into_iter().collect::<Vec<_>>();
        if values.is_empty() || values[0] == "c" {
            // empty or comment
            continue;
        }
        if num_variable.is_none() && num_clause.is_none() {
            //p cnf 5 3
            if values[0] == "p" && values[1] == "cnf" && values.len() == 4 {
                num_variable = values[2].parse::<u32>().ok();
                num_clause = values[3].parse::<u32>().ok();
                continue;
            }
        }

        let mut ok = true;
        let lits: Vec<_> = values
            .into_iter()
            .filter_map(|x| {
                if let Ok(x) = x.parse::<i32>() {
                    Some(x)
                } else {
                    ok = false;
                    None
                }
            })
            .take_while(|x| *x != 0)
            .collect();
        if !ok || lits.is_empty() {
            // skip an invalid/empty line
            continue;
        }
        let clause: Vec<Lit> = lits.iter().map(|&x| Lit::from(x)).collect();
        clauses.push(clause);
    }
    Ok(CnfData {
        num_variable,
        num_clause,
        clauses,
    })
}
