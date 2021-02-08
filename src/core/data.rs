use crate::{
    clause::alloc::CRef,
    collections::idxvec::VarVec,
    types::{bool::LitBool, lit::Lit, var::Var},
};

/// VarData has basic information that is used for searching
pub struct VarData {
    /// assignments for each variable
    assigns: VarVec<LitBool>,
    /// decision level
    level: VarVec<u32>,
    /// CRef points a clause forces to assign a var.
    reason: VarVec<CRef>,
}

impl VarData {
    pub fn new() -> VarData {
        VarData {
            assigns: VarVec::new(),
            level: VarVec::new(),
            reason: VarVec::new(),
        }
    }
    pub fn num_var(&self) -> usize {
        self.assigns.len()
    }
    pub fn new_var(&mut self) {
        self.assigns.push(LitBool::default());
        self.level.push(0);
        self.reason.push(CRef::UNDEF);
    }

    pub fn assign(&mut self, var: Var, lb: LitBool, level: u32, reason: CRef) {
        self.assigns[var] = lb;
        self.level[var] = level;
        self.reason[var] = reason;
    }

    pub fn eval(&self, lit: Lit) -> LitBool {
        LitBool::from(self.assigns[lit.var()] as i8 ^ lit.neg() as i8)
    }

    pub fn level(&self, var: Var) -> u32 {
        self.level[var]
    }

    pub fn reason(&self, var: Var) -> CRef {
        self.reason[var]
    }
}
