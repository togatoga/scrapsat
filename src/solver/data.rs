use crate::{collections::idxvec::VarVec, types::bool::LitBool};

use super::assign::AssignTrail;

/// SearchData has basic information that is used for searching
#[derive(Debug, Default)]
pub struct SearchData {
    /// assignments for each variable
    assigns: VarVec<LitBool>,
    /// decision level
    level: VarVec<u32>,
    //reason: VarVec<CRef>
    trail: AssignTrail,
}

impl SearchData {
    fn new(n: usize) -> SearchData {
        SearchData {
            assigns: VarVec::new(),
            level: VarVec::new(),
            trail: AssignTrail::new(),
        }
    }
}
