use assign::AssignTrail;
use data::VarData;

use crate::{
    clause::{alloc::CRef, db::ClauseDB},
    types::{bool::LitBool, lit::Lit},
};

mod assign;
mod data;

pub enum SatResult {
    Sat,
    Unsat,
    Unknown,
}

impl Default for SatResult {
    fn default() -> Self {
        SatResult::Unknown
    }
}

pub struct Solver {
    db: ClauseDB,
    trail: AssignTrail,
    vardata: VarData,
    result: SatResult,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            db: ClauseDB::new(),
            trail: AssignTrail::new(),
            vardata: VarData::new(),
            result: SatResult::Unknown,
        }
    }

    fn simplify_clause(&self, lits: &[Lit]) -> (bool, Vec<Lit>) {
        let mut lits = lits.to_vec();
        lits.sort();
        let mut len = 0;
        for i in 0..lits.len() {
            let mut remove = false;
            if i >= 1 {
                // x0 v !x0 means a clause is already satisfied.
                // you don't need to add it.
                if lits[i] == !lits[i - 1] {
                    return (true, lits);
                }
                // x0 v x0 duplicated
                if lits[i] == lits[i - 1] {
                    remove = true;
                }
            }
            //already assigned
            match self.vardata.eval(lits[i]) {
                LitBool::True => {
                    // a clause is already satisfied. You don't need to add it.
                    return (true, lits);
                }
                LitBool::False => {
                    // a literal is already false. You can remove it from a clause.
                    remove = true;
                }
                _ => {}
            }

            if !remove {
                lits[len] = lits[i];
                len += 1;
            }
        }
        lits.truncate(len);
        (false, lits)
    }

    fn enqueue(&mut self, lit: Lit, reason: CRef) {
        debug_assert!(self.vardata.eval(lit) == LitBool::UnDef);
        self.vardata.assign(
            lit.var(),
            lit.true_lbool(),
            self.trail.decision_level(),
            reason,
        );
        self.trail.push(lit);
    }

    pub fn add_clause(&mut self, lits: &[Lit]) {
        debug_assert!(self.trail.decision_level() == 0);
        lits.iter().for_each(|lit| {
            while lit.var().val() >= self.vardata.num_var() as u32 {
                self.vardata.new_var();
            }
        });

        let (skip, lits) = self.simplify_clause(lits);

        if skip {
            return;
        }

        if lits.is_empty() {
            self.result = SatResult::Unsat;
            return;
        } else if lits.len() == 1 {
            if self.vardata.eval(lits[0]) == LitBool::False {
                self.result = SatResult::Unsat;
                return;
            }
            self.enqueue(lits[0], CRef::UNDEF);
        } else {
        }
    }
}
