use crate::solver::Solver;
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

pub fn parse_dimacs_file(
    input_cnf_file: &std::fs::File,
    solver: &mut Solver,
    strict: Option<bool>,
) -> Result<(), failure::Error> {
    let reader = std::io::BufReader::new(input_cnf_file);
    let mut parsed_vars: Option<i32> = None;
    let mut parsed_clauses: Option<i32> = None;
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

                //skip it until char is digit
                while idx < line.len() {
                    if line[idx].is_digit(10) {
                        break;
                    } else {
                        idx += 1;
                    }
                }
                parsed_vars = Some(parse_int(&line, &mut idx)?);
                parsed_clauses = Some(parse_int(&line, &mut idx)?);
                break;
            } else {
                //NOTE
                //ReadClause
                break;
            }
        }
    }

    println!("{:?} {:?}", parsed_vars, parsed_clauses);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::dimacs::*;

    #[test]
    fn test_parse_int() {
        for num in -1000..1000 {
            let num_chars: Vec<char> = num.to_string().chars().collect();
            let mut idx = 0;
            let parsed_num = parse_int(&num_chars, &mut idx).unwrap();
            assert_eq!(parsed_num, num);
        }
    }
}
