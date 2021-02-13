#[cfg(test)]
mod tests {

    use scrapsat::types::lit::Lit;
    use scrapsat::{
        core::{SatResult, Solver},
        parser,
        types::bool::LitBool,
    };
    use walkdir::WalkDir;
    fn sat_model_check(clauses: &[Vec<Lit>], assigns: &[LitBool]) -> bool {
        for clause in clauses.iter() {
            let mut satisfied = false;
            for lit in clause {
                match assigns[lit.var().0 as usize] {
                    LitBool::True => {
                        if lit.pos() {
                            satisfied = true;
                            break;
                        }
                    }
                    LitBool::False => {
                        if lit.neg() {
                            satisfied = true;
                            break;
                        }
                    }
                    _ => {}
                };
            }
            if !satisfied {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    fn clauses_to_cnf(clauses: &[Vec<Lit>], output_file_name: &str) -> std::io::Result<()> {
        use std::io::prelude::*;

        let mut f = std::fs::File::create(output_file_name)?;
        let mut var_num = 0;
        clauses.iter().for_each(|clause| {
            for c in clause.iter() {
                var_num = std::cmp::max(var_num, c.var().0 + 1);
            }
        });
        writeln!(f, "p cnf {} {}", var_num, clauses.len())?;
        for clause in clauses.iter() {
            let line = clause
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let v = if x.pos() {
                        format!("{}", x.var().0 + 1)
                    } else {
                        format!("-{}", x.var().0 + 1)
                    };
                    if i == clause.len() - 1 {
                        format!("{} 0", v)
                    } else {
                        format!("{} ", v)
                    }
                })
                .collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
    fn test_all_files(which: &str) {
        let expected = if which == "sat" {
            SatResult::Sat
        } else {
            SatResult::Unsat
        };
        let skip_cnfs = vec![
            "cnf/sat/cnfgen_php_30_50.cnf",
            "cnf/unsat/graph_color_unsat.cnf",
        ];
        let entries = WalkDir::new(format!("cnf/{}/", which));
        for entry in entries
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let path_str = entry.path().to_str().unwrap();
            if skip_cnfs.contains(&path_str) {
                continue;
            }
            if path_str.ends_with(".cnf") {
                //parse cnf file
                let input = std::fs::File::open(path_str).unwrap();
                let cnf = parser::parse_cnf(input).unwrap();
                let mut solver = Solver::default();
                cnf.clauses
                    .iter()
                    .for_each(|clause| solver.add_clause(clause));

                eprintln!("Solving... {}", path_str);
                // Time limit is 10 sec
                let result = solver.solve();

                //Time out
                if result == SatResult::Unknown {
                    eprintln!("Skip!!(TIME LIMIT EXCEEDED): {}", path_str);
                    continue;
                }
                if result != expected {
                    eprintln!(
                        "cnf: {}, Result: {:?} Expected: {:?}",
                        path_str, result, expected
                    );
                    assert!(false);
                }
                if result == SatResult::Sat {
                    if !sat_model_check(&cnf.clauses, &solver.models) {
                        eprintln!(
                            "Assignments are wrong!! cnf: {}, Result: {:?} Expected: {:?}",
                            path_str, result, expected
                        );
                        assert!(false);
                    }
                }
            }
        }
    }
    #[test]
    fn test_solve() {
        test_all_files("sat");
        test_all_files("unsat");
    }
}
