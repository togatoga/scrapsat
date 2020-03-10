use crate::lit::Lit;
use crate::Var;

pub type LitVec<V> = IdxVec<Lit, V>;
pub type VarVec<V> = IdxVec<Var, V>;

pub trait Idx {
    fn idx(&self) -> usize;
}

#[derive(Default)]
pub struct IdxVec<T: Idx, V> {
    data: Vec<V>,
    pha: std::marker::PhantomData<T>,
}

impl<T: Idx, V: Default> IdxVec<T, V> {
    pub fn new() -> IdxVec<T, V> {
        IdxVec {
            data: Vec::new(),
            pha: std::marker::PhantomData,
        }
    }
    pub fn init(&mut self, v: T) {
        while self.data.len() <= v.idx() {
            self.data.push(V::default());
        }
    }

    pub fn iter(&self) -> std::slice::Iter<V> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<V> {
        self.data.iter_mut()
    }
}

impl<T: Idx, V> std::ops::Index<T> for IdxVec<T, V> {
    type Output = V;
    fn index(&self, t: T) -> &V {
        &self.data[t.idx()]
    }
}
impl<T: Idx, V> std::ops::IndexMut<T> for IdxVec<T, V> {
    fn index_mut(&mut self, t: T) -> &mut V {
        &mut self.data[t.idx()]
    }
}
