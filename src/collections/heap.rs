use crate::types::var::Var;

use super::idxvec::VarVec;

#[derive(Debug)]
pub struct Heap {
    heap: Vec<Var>,
    indices: VarVec<Option<usize>>,
    pub activity: VarVec<f64>,
    // Parameters for activity
    bump_inc: f64,
    decay_ratio: f64,
}
impl Default for Heap {
    fn default() -> Self {
        Heap {
            heap: Vec::default(),
            indices: VarVec::new(),
            activity: VarVec::new(),
            bump_inc: 1.0,
            decay_ratio: 0.95,
        }
    }
}

impl Heap {
    pub fn new() -> Heap {
        Heap::default()
    }

    fn gt(&self, left: Var, right: Var) -> bool {
        self.activity[left] > self.activity[right]
    }

    #[allow(dead_code)]
    fn top(self) -> Option<Var> {
        if self.heap.is_empty() {
            return None;
        }
        Some(self.heap[0])
    }

    #[allow(dead_code)]
    fn update(&mut self, v: Var) {
        if !self.in_heap(v) {
            self.push(v);
        } else {
            let idx = self.indices[v].unwrap();
            self.up(idx);
            self.down(idx);
        }
    }
    pub fn up(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let mut idx = i;
        let x = self.heap[idx];
        let mut par = (idx - 1) >> 1;
        loop {
            if !self.gt(x, self.heap[par]) {
                break;
            }
            self.heap[idx] = self.heap[par];
            self.indices[self.heap[par]] = Some(idx);
            idx = par;
            if idx == 0 {
                break;
            }
            par = (par - 1) >> 1;
        }
        self.heap[idx] = x;
        self.indices[x] = Some(idx);
    }

    pub fn pop(&mut self) -> Option<Var> {
        if self.heap.is_empty() {
            return None;
        }
        let x = self.heap[0];
        self.indices[x] = None;
        if self.heap.len() > 1 {
            self.heap[0] = *self.heap.last().unwrap();
            self.indices[self.heap[0]] = Some(0);
        }
        self.heap.pop();
        if self.heap.len() > 1 {
            self.down(0);
        }
        Some(x)
    }

    fn down(&mut self, i: usize) {
        let x = self.heap[i];
        let mut idx = i;
        while 2 * idx + 1 < self.heap.len() {
            let left = 2 * idx + 1;
            let right = left + 1;
            let child = if right < self.heap.len() && self.gt(self.heap[right], self.heap[left]) {
                right
            } else {
                left
            };
            if self.gt(self.heap[child], x) {
                self.heap[idx] = self.heap[child];
                self.indices[self.heap[idx]] = Some(idx);
                idx = child;
            } else {
                break;
            }
        }
        self.heap[idx] = x;
        self.indices[x] = Some(idx);
    }

    pub fn push(&mut self, v: Var) {
        if self.in_heap(v) {
            return;
        }
        while (v.0 as usize) >= self.indices.len() {
            self.indices.push(None);
            self.activity.push(0.0);
        }
        self.indices[v] = Some(self.heap.len());
        self.heap.push(v);
        self.up(self.indices[v].expect("No index"));
    }

    pub fn in_heap(&mut self, v: Var) -> bool {
        (v.0 as usize) < self.indices.len() && self.indices[v].is_some()
    }

    pub fn decay(&mut self) {
        self.bump_inc /= self.decay_ratio;
    }
    
    pub fn bump_activity(&mut self, v: Var) {
        self.activity[v] += self.bump_inc;

        if self.activity[v] >= 1e100 {
            for act in self.activity.iter_mut() {
                *act *= 1e-100;
            }
            self.bump_inc *= 1e-100;
        }
        if self.in_heap(v) {
            let idx = self.indices[v].expect("No index");
            self.up(idx);
        }
    }
}
