use crate::lit::Lit;
pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Solver {}
    }

    pub fn add_clause(mut lits: Vec<Lit>) {
        lits.sort();
        lits.dedup();
    }
}
