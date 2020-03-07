use crate::lit::Lit;
pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Solver {}
    }

    pub fn add_clause(&self, mut lits: Vec<Lit>) {
        //check decision_level == 0;
        lits.sort();
        lits.dedup();
    }
}
