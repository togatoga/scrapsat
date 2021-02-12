use crate::{clause::alloc::CRef, collections::idxvec::VarVec, types::lit::Lit};
/// `Analyzer` has a bunch of data that is used in `analyze`.
pub struct Analyzer {
    pub seen: VarVec<bool>,
    ccmin_stack: Vec<CRef>,
    ccmin_clear: Vec<Lit>,
    pub analyze_toclear: Vec<Lit>,
    pub learnt_clause: Vec<Lit>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            seen: VarVec::new(),
            ccmin_stack: Vec::new(),
            ccmin_clear: Vec::new(),
            analyze_toclear: Vec::new(),
            learnt_clause: Vec::new(),
        }
    }
}
