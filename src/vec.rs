// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate blas;
extern crate num;

use std::num::NumCast;
use std::vec::Vec as StdVec;
use self::blas::{
    Vector,
    VectorOperations,
};
use self::num::complex::{
    Complex32,
    Complex64,
};

pub struct Vec<T> {
    inc: i32,
    data: StdVec<T>,
}

impl<T> Vector<T> for Vec<T> {
    #[inline]
    fn inc(&self) -> i32 { self.inc }

    #[inline]
    fn len(&self) -> i32 {
        let l: Option<i32> = NumCast::from(self.data.len());
        l.unwrap()
    }

    #[inline]
    fn as_ptr(&self) -> *const T { self.data.as_slice().as_ptr() }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T { self.data.as_mut_slice().as_mut_ptr() }
}

impl VectorOperations<f32> for Vec<f32> {}
impl VectorOperations<f64> for Vec<f64> {}
impl VectorOperations<Complex32> for Vec<Complex32> {}
impl VectorOperations<Complex64> for Vec<Complex64> {}
