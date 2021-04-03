// original: https://github.com/uazu/qcell/blob/master/src/lcell.rs

// Copyright (c) 2019 Jim Peters
// Copyright (c) 2019 `qcell` crate contributors
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::cell::UnsafeCell;
use std::marker::PhantomData;

type Id<'id> = PhantomData<*mut &'id ()>;

pub struct LCellOwner<'id> {
    _id: Id<'id>,
}

impl<'id> LCellOwner<'id> {
    pub fn scope<R>(f: impl for<'a> FnOnce(LCellOwner<'a>) -> R) -> R {
        f(Self { _id: PhantomData })
    }
    pub fn cell<T>(&self, value: T) -> LCell<'id, T> { LCell::new(value) }
    pub fn get<'a, T>(&'a self, lc: &'a LCell<'id, T>) -> &'a T {
        unsafe { &*lc.value.get() }
    }
    pub fn get_mut<'a, T>(&'a mut self, lc: &'a LCell<'id, T>) -> &'a mut T {
        unsafe { &mut *lc.value.get() }
    }
    pub fn get_mut_2<'a, T, U>(
        &'a mut self,
        lc1: &'a LCell<'id, T>,
        lc2: &'a LCell<'id, U>,
    ) -> (&'a mut T, &'a mut U) {
        assert!(
            lc1 as *const _ as usize != lc2 as *const _ as usize,
            "Illegal to borrow same LCell twice with get_mut_2()"
        );
        unsafe { (&mut *lc1.value.get(), &mut *lc2.value.get()) }
    }
    pub fn get_mut_3<'a, T, U, V>(
        &'a mut self,
        lc1: &'a LCell<'id, T>,
        lc2: &'a LCell<'id, U>,
        lc3: &'a LCell<'id, V>,
    ) -> (&'a mut T, &'a mut U, &'a mut V) {
        assert!(
            (lc1 as *const _ as usize != lc2 as *const _ as usize)
                && (lc2 as *const _ as usize != lc3 as *const _ as usize)
                && (lc3 as *const _ as usize != lc1 as *const _ as usize),
            "Illegal to borrow same LCell twice with get_mut_3()"
        );
        unsafe { (&mut *lc1.value.get(), &mut *lc2.value.get(), &mut *lc3.value.get()) }
    }
}

pub struct LCell<'id, T> {
    _id: Id<'id>,
    value: UnsafeCell<T>,
}

impl<'id, T> LCell<'id, T> {
    pub fn new(value: T) -> LCell<'id, T> {
        LCell { _id: PhantomData, value: UnsafeCell::new(value) }
    }
}

unsafe impl<'id> Sync for LCellOwner<'id> {}
unsafe impl<'id, T: Send + Sync> Sync for LCell<'id, T> {}
