pub mod assignments;
pub mod clause;
pub mod dimacs;
pub mod index_vec;
pub mod lit;
pub mod solver;
pub mod watcher;
#[macro_use]
extern crate failure;

use crate::index_vec::Idx;
use crate::lit::Lit;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Var(i32);
impl Idx for Var {
    fn idx(&self) -> usize {
        self.0 as usize
    }
}

pub fn var_to_lit(v: Var) -> Lit {
    debug_assert!(v.0 != 0);
    let neg = v.0 < 0;
    Lit::new(v, neg)
}
