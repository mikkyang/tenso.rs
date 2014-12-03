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
use self::blas::matrix_vector::ops::{
    Ger,
    Gerc,
};
use self::blas::vector::ops::{
    Axpy,
    Copy,
    Dot,
    Dotc,
    Scal,
};
use Trans;
use mat::Mat;

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
    fn len(&self) -> i32 { NumCast::from(self.data.len()).unwrap() }

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

impl<T, V> Add<V, Vec<T>> for Vec<T>
where T: Axpy + Copy + Default,
      V: Vector<T>,
{
    fn add(&self, x: &V) -> Vec<T> {
        let mut result = self.clone();
        Axpy::axpy(&Default::one(), x, &mut result);
        result
    }
}

impl<T> Mul<T, Vec<T>> for Vec<T>
where T: Copy + Scal,
{
    fn mul(&self, alpha: &T) -> Vec<T> {
        let mut result = self.clone();
        Scal::scal(alpha, &mut result);
        result
    }
}

impl<T, V> Mul<Trans<V>, Mat<T>> for Vec<T>
where T: Copy + Default + Ger + Gerc,
      V: Vector<T>,
{
    fn mul(&self, x: &Trans<V>) -> Mat<T> {
        let v = x.into_inner();
        let rows = self.data.len();
        let cols: uint = NumCast::from(v.len()).unwrap();
        let mut result = Mat::zero(rows, cols);

        match x {
            &Trans::T(_) => Ger::ger(&Default::one(), self, v, &mut result),
            &Trans::H(_) => Gerc::gerc(&Default::one(), self, v, &mut result),
        }

        result
    }
}

impl<T, V> Mul<V, T> for Trans<Vec<T>>
where T: Copy + Dot + Dotc,
      V: Vector<T>,
{
    fn mul(&self, x: &V) -> T {
        match *self {
            Trans::T(ref v) => Dot::dot(v, x),
            Trans::H(ref v) => Dotc::dotc(v, x),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate num;

    use self::num::complex::Complex;
    use vec::Vec;
    use Trans::{
        T,
        H,
    };

    #[test]
    fn add() {
        let x = Vec::from_vec(vec![1f32, 2f32]);
        let y = vec![-1f32, 2f32];

        assert_eq!((x + y).as_vec(), &vec![0f32, 4f32]);
    }

    #[test]
    fn scalar_mul() {
        let x = Vec::from_vec(vec![1f32, 2f32]);

        assert_eq!((x * 3.0).as_vec(), &vec![3f32, 6f32]);
    }

    #[test]
    fn dot_mul() {
        let x = Vec::from_vec(vec![1f32, 2f32]);
        let y = vec![-1f32, 2f32];

        assert_eq!(T(x) * y, 3.0);
    }

    #[test]
    fn complex_conj() {
        let x = Vec::from_vec(vec![Complex::new(1f32, -1f32), Complex::new(1f32, -3f32)]);
        let y = vec![Complex::new(1f32, 2f32), Complex::new(1f32, 3f32)];

        assert_eq!(H(x) * y, Complex::new(-9f32, 9f32));
    }
}
