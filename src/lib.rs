pub mod assignments;
pub mod clause;
pub mod dimacs;
pub mod index_vec;
pub mod lit;
pub mod solver;
pub mod watcher;
#[macro_use]
extern crate failure;

use crate::lit::Lit;

pub type Var = i32;

pub fn var_to_lit(v: Var) -> Lit {
    debug_assert!(v != 0);
    let neg = v < 0;
    Lit::new(v, neg)
}
