pub mod dimacs;
pub mod lit;
pub mod solver;
#[macro_use]
extern crate failure;

pub type Var = i32;
