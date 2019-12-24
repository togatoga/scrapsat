mod alloc;
pub mod db;
mod word;

use alloc::CRef;
use core::slice;
use std::ops::{Index, IndexMut};
use word::Flags;

use word::ClauseWord;

use crate::types::lit::Lit;

pub struct ClauseIter<'a>(slice::Iter<'a, ClauseWord>);

impl<'a> Iterator for ClauseIter<'a> {
    type Item = &'a Lit;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|word| unsafe { &word.lit })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
pub struct ClauseIterMut<'a>(slice::IterMut<'a, ClauseWord>);
impl<'a> Iterator for ClauseIterMut<'a> {
    type Item = &'a mut Lit;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|word| unsafe { &mut word.lit })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// NOTE
/// All `ClauseWord` must be `lit` only if flags isn't `RELOCATED`.
/// If flags is `RELOCATED`, data[0] must be `relocation`.
pub struct Clause<'a> {
    flags: &'a mut Flags,
    data: &'a mut [ClauseWord],
    extra: Option<&'a mut ClauseWord>,
}

impl<'a> Clause<'a> {
    fn len(&self) -> usize {
        self.data.len() as usize
    }
    fn learnt(&self) -> bool {
        self.flags.contains(Flags::LEARNT)
    }
    fn activity(&self) -> f32 {
        self.flags.contains(Flags::LEARNT);
        unsafe { self.extra.as_ref().expect("No extra").activity }
    }
    fn relocated(&self) -> bool {
        self.flags.contains(Flags::RELOCATED)
    }
    fn relocate(&mut self, cref: CRef) {
        debug_assert!(!self.flags.contains(Flags::RELOCATED));
        self.flags.insert(Flags::RELOCATED);
        self.data[0].relocation = cref;
    }
    fn relocation(&self) -> CRef {
        debug_assert!(self.flags.contains(Flags::RELOCATED));
        unsafe { self.data[0].relocation }
    }
    fn iter(&self) -> ClauseIter {
        debug_assert!(!self.flags.contains(Flags::RELOCATED));
        ClauseIter(self.data.iter())
    }
    fn iter_mut(&mut self) -> ClauseIterMut {
        debug_assert!(!self.flags.contains(Flags::RELOCATED));
        ClauseIterMut(self.data.iter_mut())
    }
}

impl<'a> Index<usize> for Clause<'a> {
    type Output = Lit;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.data[index].lit }
    }
}

impl<'a> IndexMut<usize> for Clause<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.data[index].lit }
    }
}
