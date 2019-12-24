use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::types::{lit::Lit, var::Var};

pub type LitVec<V> = IdxVec<Lit, V>;
pub type VarVec<V> = IdxVec<Var, V>;

pub trait Idx {
    fn idx(&self) -> usize;
}

#[derive(Debug, Default)]
pub struct IdxVec<T: Idx, V> {
    data: Vec<V>,
    _markder: PhantomData<T>,
}

impl<T: Idx, V> IdxVec<T, V> {
    pub fn new() -> IdxVec<T, V> {
        IdxVec {
            data: Vec::new(),
            _markder: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn push(&mut self, x: V) {
        self.data.push(x);
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Trait Implementation for `Lit` and `Var`

impl<T: Idx, V> Deref for IdxVec<T, V> {
    type Target = [V];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Idx, V> DerefMut for IdxVec<T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Lit
impl Idx for Lit {
    fn idx(&self) -> usize {
        debug_assert!(
            self.val() >= 0,
            format!("A value is negative {}", self.val())
        );
        self.val() as usize
    }
}
impl<Lit: Idx, V> Index<Lit> for LitVec<V> {
    type Output = V;
    fn index(&self, lit: Lit) -> &Self::Output {
        &self.data[lit.idx()]
    }
}

impl<Lit: Idx, V> IndexMut<Lit> for LitVec<V> {
    fn index_mut(&mut self, lit: Lit) -> &mut Self::Output {
        &mut self.data[lit.idx()]
    }
}
/// Var

impl Idx for Var {
    fn idx(&self) -> usize {
        debug_assert!(
            self.val() >= 0,
            format!("A value is negative {}", self.val())
        );
        self.val() as usize
    }
}

impl<Var: Idx, V> Index<Var> for VarVec<V> {
    type Output = V;
    fn index(&self, var: Var) -> &Self::Output {
        &self.data[var.idx()]
    }
}

impl<Var: Idx, V> IndexMut<Var> for VarVec<V> {
    fn index_mut(&mut self, var: Var) -> &mut Self::Output {
        &mut self.data[var.idx()]
    }
}

#[cfg(test)]
mod tests {

    use crate::types::{lit::Lit, var::Var};

    use super::{LitVec, VarVec};

    #[test]
    fn test_idxvec() {
        {
            // Var
            let mut values = VarVec::new();
            for v in (0..10).map(Var::from_idx) {
                values.push(v.val());
            }

            for (i, v) in (0..10).map(Var::from_idx).enumerate() {
                let x = values[v];
                assert_eq!(x, i as i32);
            }
        }
        {
            // Lit
            let mut values = LitVec::new();
            for v in (0..10).map(Var::from_idx) {
                let x = Lit::from(v);
                values.push(x.val());
            }

            for (i, lit) in (0..10).map(Var::from_idx).map(Lit::from).enumerate() {
                let x = values[lit];
                assert_eq!(x, i as i32);
            }
            assert!(values.len() == 10);
        }
    }
}
