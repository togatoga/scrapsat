use crate::lit::{Lit, LitBool};
pub struct Solver {
    polarity: Vec<LitBool>, // The preferred polarity of each variable
    assigns: Vec<LitBool>,  // The current number of variables.
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            polarity: Vec::new(),
            assigns: Vec::new(),
        }
    }

    // n_var returns the current number of variables.
    pub fn n_var(&self) -> usize {
        self.assigns.len()
    }
    pub fn new_var(&mut self, sign: bool) {
        self.assigns.push(LitBool::Undef);
    }
    pub fn add_clause(&mut self, mut lits: Vec<Lit>) {
        //Reserve the space of variables
        lits.iter().for_each(|lit| {
            let var = lit.var().abs();
            while var as usize >= self.n_var() {
                self.new_var(lit.neg());
            }
        });

        //check decision_level == 0;
        lits.sort();
        lits.dedup();
    }
}
