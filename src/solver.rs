use crate::assignments::Assignment;
use crate::clause::ClauseRef;
use crate::lit::{Lit, LitBool};
use crate::watcher::Watches;

pub struct Solver {
    assignment: Assignment,
    watches: Watches,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            assignment: Assignment::new(),
            watches: Watches::new(),
        }
    }

    pub fn n_var(&self) -> usize {
        self.assignment.n_var()
    }

    // n_var returns the current number of variables.
    pub fn add_clause(&mut self, mut lits: Vec<Lit>) {
        //Reserve the space of variables
        lits.iter().for_each(|lit| {
            let var = lit.var().abs();
            while var as usize >= self.assignment.n_var() {
                self.assignment.new_var();
                self.watches.init_var(var);
            }
        });

        //check decision_level == 0;
        lits.sort();
        lits.dedup();
    }
}
