/// A Variable(`Var`) is a boolean variable.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Var(pub u32);
impl Var {
    pub const UNDEF: Var = Var(!0);
    #[allow(dead_code)]
    pub fn from_idx(x: usize) -> Var {
        Var(x as u32)
    }
    pub fn val(&self) -> u32 {
        self.0
    }
}

impl Default for Var {
    fn default() -> Self {
        Var::UNDEF
    }
}
