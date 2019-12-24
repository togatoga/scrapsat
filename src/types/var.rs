/// A Variable(`Var`) is a boolean variable.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Var(pub i32);
impl Var {
    pub const UNDEF: Var = Var(-1);
    #[allow(dead_code)]
    pub fn from_idx(x: usize) -> Var {
        Var(x as i32)
    }
    pub fn val(&self) -> i32 {
        self.0
    }
}

impl Default for Var {
    fn default() -> Self {
        Var::UNDEF
    }
}
