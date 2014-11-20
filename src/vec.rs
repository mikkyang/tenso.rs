// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate blas;
extern crate num;

use std::num::NumCast;
use std::ops::{
    Add,
    Mul,
};
use std::vec::Vec as StdVec;
use self::blas::Vector;
use self::blas::default::Default;
use self::blas::vector::ops::{Copy, Axpy, Scal, Dot};

pub struct Vec<T> {
    inc: i32,
    data: StdVec<T>,
}

impl<T> Vec<T> {
    pub fn from_vec(vec: StdVec<T>) -> Vec<T> {
        Vec {
            inc: 1i32,
            data: vec,
        }
    }

    pub fn as_vec(&self) -> &StdVec<T> {
        &self.data
    }
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

impl<T: Copy> Clone for Vec<T> {
    fn clone(&self) -> Vec<T> {
        let n = self.data.len();
        let mut result = Vec {
            inc: self.inc,
            data: StdVec::with_capacity(n),
        };
        Copy::copy(self, &mut result);
        unsafe { result.data.set_len(n); }

        result
    }
}

impl<T: Copy + Axpy + Default> Add<Vec<T>, Vec<T>> for Vec<T> {
    fn add(&self, x: &Vec<T>) -> Vec<T> {
        let mut result = self.clone();
        let one: T = Default::one();
        Axpy::axpy(&one, x, &mut result);
        result
    }
}

impl<T: Copy + Scal> Mul<T, Vec<T>> for Vec<T> {
    fn mul(&self, alpha: &T) -> Vec<T> {
        let mut result = self.clone();
        Scal::scal(alpha, &mut result);
        result
    }
}

impl<T: Copy + Dot> Mul<Vec<T>, T> for Vec<T> {
    fn mul(&self, x: &Vec<T>) -> T {
        Dot::dot(self, x)
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec;

    #[test]
    fn add() {
        let x = Vec::from_vec(vec![1f32, 2f32]);
        let y = Vec::from_vec(vec![-1f32, 2f32]);

        assert_eq!((x + y).as_vec(), &vec![0f32, 4f32]);
    }
}

