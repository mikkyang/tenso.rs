// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![macro_use]

use std::fmt;
use std::mem;
use std::num::NumCast;
use std::ops::{
    Index,
    Mul,
};
use std::raw::Slice;
use blas::matrix::Matrix;
use blas::matrix::ops::{
    Gemm,
};
use blas::matrix_vector::ops::{
    Gemv,
};
use blas::vector::ops::Copy;
use default::Default;

pub struct Mat<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Mat<T> {
    #[inline]
    pub fn new() -> Mat<T> {
        let _data: Vec<T> = Vec::new();
        Mat { rows: 0, cols: 0, data: _data }
    }

    #[inline]
    pub fn from_vec(rows: usize, cols: usize, vec: Vec<T>) -> Mat<T> {
        Mat { rows: rows, cols: cols, data: vec }
    }

    #[inline]
    pub fn zero(rows: usize, cols: usize) -> Mat<T> {
        let mut _data: Vec<T> = Vec::with_capacity(rows * cols);
        unsafe { _data.set_len(rows * cols); }
        Mat { rows: rows, cols: cols, data: _data}
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub unsafe fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
    }

    #[inline]
    pub unsafe fn set_cols(&mut self, cols: usize) {
        self.cols = cols;
    }

    #[inline]
    pub unsafe fn as_slice<'a>(&'a self) -> &'a [T] {
        &self.data[..]
    }

    #[inline]
    pub unsafe fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        &mut self.data[..]
    }
}

impl<T> Matrix<T> for Mat<T> {
    #[inline]
    fn rows(&self) -> i32 {
        let l: Option<i32> = NumCast::from(self.rows());
        match l {
            Some(l) => l,
            None => panic!(),
        }
    }

    #[inline]
    fn cols(&self) -> i32 {
        let l: Option<i32> = NumCast::from(self.cols());
        match l {
            Some(l) => l,
            None => panic!(),
        }
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        unsafe { self.as_slice().as_ptr() }
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        unsafe { self.as_mut_slice().as_mut_ptr() }
    }
}


impl<T: Copy> Clone for Mat<T> {
    fn clone(&self) -> Mat<T> {
        let n = self.rows * self.cols;

        let mut x = Vec::with_capacity(n);
        unsafe {
            Copy::copy(&self.data, &mut x);
            x.set_len(n);
        }

        Mat::from_vec(self.rows, self.cols, x)
    }
}

impl<T: PartialEq> PartialEq for Mat<T> {
    fn eq(&self, other: &Mat<T>) -> bool {
        self.rows == other.rows &&
        self.cols == other.cols &&
        self.data == other.data
    }
}

impl<T> Index<usize> for Mat<T> {
    type Output = [T];

    fn index<'a>(&'a self, index: usize) -> &'a [T] {
        unsafe {
            let ptr = (&self.data[..]).as_ptr().offset((index * self.cols) as isize);
            mem::transmute(Slice { data: ptr, len: self.cols })
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Mat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0usize..self.rows {
            for j in 0usize..self.cols {
                match write!(f, "{:?}", self[i][j]) {
                    Ok(_) => (),
                    x => return x,
                }
            }

            match writeln!(f, "") {
                Ok(_) => (),
                x => return x,
            }
        }

        Ok(())
    }
}

impl<'a, 'b, T> Mul<&'a Vec<T>> for &'b Mat<T>
where T: Default + Gemv {
    type Output = Vec<T>;

    fn mul(self, x: &'a Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(self.rows);

        Gemv::gemv(&Default::one(), self, x, &Default::zero(), &mut result);
        unsafe { result.set_len(self.rows); }

        result
    }
}

impl<'a, 'b, T> Mul<&'a Mat<T>> for &'b Mat<T>
where T: Default + Gemm {
    type Output = Mat<T>;

    fn mul(self, b: &'a Mat<T>) -> Mat<T> {
        let mut result = Mat::zero(self.cols, b.rows);
        Gemm::gemm(&Default::one(), self, b, &Default::zero(), &mut result);

        result
    }
}

#[macro_export]
macro_rules! mat(
    ($($($e: expr),+);*) => ({
        // leading _ to allow empty construction without a warning.
        let mut _temp = Mat::new();
        let mut rows = 0usize;
        let mut _cols;
        $(
            rows += 1;
            _cols = 0usize;
            $(
                _cols += 1;
                _temp.push($e);
            )+
        )*

        unsafe {
            _temp.set_rows(rows);
            _temp.set_cols(_cols);
        }

        _temp
    });
);

#[cfg(test)]
mod tests {
    extern crate test;

    use mat::Mat;

    #[test]
    fn index() {
        let a = mat![1f32, 2f32];
        assert_eq!(1.0, a[0][0]);
        assert_eq!(2.0, a[0][1]);

        let b = mat![1f32; 2f32];
        assert_eq!(1.0, b[0][0]);
        assert_eq!(2.0, b[1][0]);

        let m = mat![1f32, 2f32; 3f32, 4f32];
        assert_eq!(1.0, m[0][0]);
        assert_eq!(2.0, m[0][1]);
        assert_eq!(3.0, m[1][0]);
        assert_eq!(4.0, m[1][1]);
    }

    #[test]
    fn mul_vec() {
        let a = mat![1f32, -2f32; 2f32, -4f32];
        let x = vec![2f32, 1f32];

        assert_eq!(&a * &x, vec![0f32, 0f32]);
    }

    #[test]
    fn mul_mat() {
        let a = mat![1f32, -2f32; 2f32, -4f32];
        let b = mat![1f32, -2f32; 2f32, -4f32];

        let result = mat![-3f32, 6f32; -6f32, 12f32];
        assert_eq!(&a * &b, result);
    }
}
