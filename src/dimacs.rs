use crate::lit::Lit;
use crate::solver::AddClauseResult;
use crate::solver::Solver;
use crate::Var;
use std::io::BufRead;

fn parse_int(input: &[char], idx: &mut usize) -> Result<i32, failure::Error> {
    skip_whitespace(input, idx);
    if *idx >= input.len() {
        return Err(format_err!("Empty"));
    }
    let neg = match input[*idx] {
        '-' => {
            *idx += 1;
            true
        }
        '+' => {
            *idx += 1;
            false
        }
        _ => false,
    };

    if *idx >= input.len() {
        return Err(format_err!("Empty"));
    }
    let mut value = 0;
    while *idx < input.len() {
        let c = input[*idx];
        if c.is_whitespace() {
            *idx += 1;
            break;
        }
        if let Some(num) = c.to_digit(10) {
            value = 10 * value + num as i32;
        } else {
            *idx += 1;
            return Err(format_err!("Invalid Number {}", input[*idx]));
        }
        *idx += 1;
    }
    let value = if neg { -value } else { value };
    Ok(value)
}

fn parse_vars_and_clauses(input: &[char], idx: &mut usize) -> Result<(i32, i32), failure::Error> {
    //p cnf <variable num> <clause num>
    //e.g. p cnf 90 300
    skip_whitespace(input, idx);
    if *idx >= input.len() {
        return Err(format_err!("Parse failure {:?}", input));
    }
    assert_eq!(input[*idx], 'p');
    *idx += 1;
    skip_whitespace(input, idx);
    if *idx + 2 < input.len() {
        let cnf: String = input[*idx..*idx + 3].iter().collect();
        if &cnf != "cnf" {
            return Err(format_err!("cnf is not found {:?}", input));
        } else {
            *idx += 3;
        }
    }
    skip_whitespace(input, idx);
    let parsed_vars = parse_int(input, idx)?;
    let parsed_clauses = parse_int(input, idx)?;

    Ok((parsed_vars, parsed_clauses))
}

fn skip_whitespace(input: &[char], idx: &mut usize) {
    while *idx < input.len() {
        let c = input[*idx];
        if c.is_whitespace() {
            *idx += 1;
            continue;
        } else {
            break;
        }
    }
}

fn parse_clause(input: &[char], idx: &mut usize) -> Result<Vec<Lit>, failure::Error> {
    let mut lits: Vec<Lit> = vec![];
    loop {
        skip_whitespace(input, idx);
        let parsed_lit = parse_int(input, idx)?;
        //A clause is supposed to be end with zero
        //e.g.
        //13 14 15 0
        if parsed_lit == 0 {
            skip_whitespace(input, idx);
            break;
        }
        let neg = parsed_lit < 0;
        let var = Var(parsed_lit.abs() - 1);
        lits.push(Lit::new(var, neg));
    }
    if input.len() != *idx {
        return Err(format_err!("PARSE ERROR WRONG FORMAT: {:?}", input));
    }
    Ok(lits)
}
pub fn parse_dimacs_file(
    input_cnf_file: &std::fs::File,
    solver: &mut Solver,
    strict: Option<bool>,
) -> Result<(), failure::Error> {
    let reader = std::io::BufReader::new(input_cnf_file);
    let mut parsed_header_vars: Option<i32> = None;
    let mut parsed_header_clauses: Option<i32> = None;
    let mut clause_cnt = 0;
    for line in reader.lines() {
        let line: Vec<char> = line?.chars().collect();
        //line is empty or comment. skip
        if line.is_empty() || line[0] == 'c' {
            continue;
        }
        //char by char
        let mut idx: usize = 0;
        while idx < line.len() {
            let c = line[idx];
            if c.is_whitespace() || c == 'c' {
                idx += 1;
                continue;
            }
            if c == 'p' {
                //p cnf <variable num> <clause num>
                //e.g. p cnf 90 300
                match parse_vars_and_clauses(&line, &mut idx) {
                    Ok((vars, clauses)) => {
                        parsed_header_vars = Some(vars);
                        parsed_header_clauses = Some(clauses);
                    }
                    _ => {
                        //skip line
                        break;
                    }
                }
            } else {
                let lits = parse_clause(&line, &mut idx)?;
                match solver.add_clause(&lits) {
                    AddClauseResult::UnSAT => return Ok(()),
                    _ => {}
                }
                clause_cnt += 1;
            }
        }
    }

    if strict.unwrap_or(false) {
        if let Some(header_clause) = parsed_header_clauses {
            if clause_cnt != header_clause {
                return Err(format_err!("PARSE ERROR! DIMACS header mismatch: wrong number of clauses. header clause: {} clause: {}", header_clause, clause_cnt));
            }
        }
        if let Some(header_var) = parsed_header_vars {
            if header_var as usize != solver.n_var() {
                return Err(format_err!("PARSE ERROR! DIMACS header mismatch: wrong number of variables. header variable: {} variables: {}", header_var, solver.n_var()));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dimacs::*;

    #[test]
    fn test_parse_int() {
        for num in -1000..1000 {
            let num_chars: Vec<char> = num.to_string().chars().collect();
            let mut idx = 0;
            let parsed_num = parse_int(&num_chars, &mut idx).unwrap();
            assert_eq!(parsed_num, num);
        }
        let num_chars: Vec<char> = "0000001".chars().collect();
        let mut idx = 0;
        assert_eq!(parse_int(&num_chars, &mut idx).unwrap(), 1);

        let num_chars: Vec<char> = "+810".chars().collect();
        let mut idx = 0;
        let parsed_num = parse_int(&num_chars, &mut idx).unwrap();
        assert_eq!(parsed_num, 810);

        //extra whitespace
        let num_chars: Vec<char> = " -10 ".chars().collect();
        let mut idx = 0;
        let parsed_num = parse_int(&num_chars, &mut idx).unwrap();
        assert_eq!(parsed_num, -10);

        //invalid cases
        let num_chars: Vec<char> = "123-10".chars().collect();
        let mut idx = 0;
        assert!(parse_int(&num_chars, &mut idx).is_err());
        let num_chars: Vec<char> = "----12310".chars().collect();
        let mut idx = 0;
        assert!(parse_int(&num_chars, &mut idx).is_err());
    }

    #[test]
    fn test_parse_vars_and_clauses() {
        let line: Vec<char> = "p cnf 1000 100000".to_string().chars().collect();
        let mut idx = 0;
        assert_eq!(
            parse_vars_and_clauses(&line, &mut idx).unwrap(),
            (1000, 100000)
        );
        //extra whitespace
        let line: Vec<char> = "      p             cnf     1000     100000   "
            .to_string()
            .chars()
            .collect();
        let mut idx = 0;
        assert_eq!(
            parse_vars_and_clauses(&line, &mut idx).unwrap(),
            (1000, 100000)
        );
        //invalid cases
        let line: Vec<char> = "      p             cnf     1000   "
            .to_string()
            .chars()
            .collect();
        let mut idx = 0;
        assert!(parse_vars_and_clauses(&line, &mut idx).is_err());
        let line: Vec<char> = "      p                  1000   10000"
            .to_string()
            .chars()
            .collect();
        let mut idx = 0;
        assert!(parse_vars_and_clauses(&line, &mut idx).is_err());
        let line: Vec<char> = "p    cnf".to_string().chars().collect();
        let mut idx = 0;
        assert!(parse_vars_and_clauses(&line, &mut idx).is_err());
    }

    #[test]
    fn test_parse_clause() {
        let line: Vec<char> = "1 -2 3 0".to_string().chars().collect();
        let mut idx = 0;
        let clause = parse_clause(&line, &mut idx).unwrap();

        assert_eq!(clause.len(), 3);
        assert_eq!(clause[0], Lit::new(Var(0), false));
        assert_eq!(clause[1], Lit::new(Var(1), true));
        assert_eq!(clause[2], Lit::new(Var(2), false));
        let line: Vec<char> = "1 -2 3".to_string().chars().collect();
        let mut idx = 0;
        let clause = parse_clause(&line, &mut idx);
        assert!(clause.is_err());
        let line: Vec<char> = "1 0 3 0".to_string().chars().collect();
        let mut idx = 0;
        let clause = parse_clause(&line, &mut idx);
        assert!(clause.is_err());
    }
}
