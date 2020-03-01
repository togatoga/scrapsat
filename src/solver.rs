use crate::lit::Lit;
pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Solver {}
    }

    pub fn add_clause(lits: &[Lit]) {
        let ps = {
            let mut ps = lits.to_vec();
            ps.sort();
            ps.dedup();

            ps
        };
    }
}
