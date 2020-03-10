use crate::assignments::Assignment;
use crate::index_vec::Idx;
use crate::lit::Lit;
use crate::watcher::Watches;

pub struct Solver {
    assignment: Assignment,
    watches: Watches,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

pub enum AddClauseResult {
    UnSAT,
    ClauseAlreadySatisfied,
    None,
}
impl Solver {
    pub fn new() -> Solver {
        Solver {
            assignment: Assignment::new(),
            watches: Watches::new(),
        }
    }
    // n_var returns the current number of variables.
    pub fn n_var(&self) -> usize {
        self.assignment.n_var()
    }

    pub fn add_clause(&mut self, lits: &[Lit]) -> AddClauseResult {
        //Reserve the space of variables
        lits.iter().for_each(|lit| {
            let var = lit.var();
            while var.idx() >= self.assignment.n_var() {
                self.assignment.new_var();
                self.watches.init_var(var);
            }
        });
        let lits: Vec<Lit> = {
            let mut lits = lits.to_vec();
            lits.sort();
            lits.dedup();
            let mut prev = None;
            for &lit in lits.iter() {
                if self.assignment.is_assigned_true(lit) || prev == Some(!lit) {
                    return AddClauseResult::ClauseAlreadySatisfied;
                }
                prev = Some(lit);
            }

            lits
        };
        match &lits[..] {
            [] => {
                //UNSAT
                AddClauseResult::UnSAT
            }
            [unit] => AddClauseResult::None,
            lits => AddClauseResult::None,
        }
    }
}
