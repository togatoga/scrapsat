use crate::solver::Solver;
use std::io::BufRead;

fn parse_int(input: &[char], idx: &mut usize) -> Result<i32, failure::Error> {
    let mut value = 0;

    skip_whitespace(input, idx);
    
    while *idx < input.len() {

    }

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
    let mut reader = std::io::BufReader::new(input_cnf_file);
    for line in reader.lines() {
        let line: Vec<char> = line?.chars().collect();
        //line is empty or comment. skip
        if line.is_empty() || line[0] == 'c' {
            continue;
        }

        let mut idx: usize = 0;
        while idx < line.len() {
            let c = line[idx];
            if c.is_whitespace() || c == 'c' {
                idx += 1;
                continue;
            }
            println!("{:?}", &line[idx..line.len()]);
            //parseInt
            idx += 1;
        }
        println!("");
    }

    Ok(())
}
