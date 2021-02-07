use assign::AssignTrail;
use data::VarData;

use crate::clause::db::ClauseDB;

mod assign;
mod data;
pub struct Solver {
    db: ClauseDB,
    trail: AssignTrail,
    vardata: VarData,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            db: ClauseDB::new(),
            trail: AssignTrail::new(),
            vardata: VarData::new(),
        }
    }
}
