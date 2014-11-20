// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![macro_escape]

extern crate blas;

use std::fmt;
use std::mem;
use std::num::NumCast;
use std::ops::Index;
use std::raw::Slice;
use std::slice::AsSlice;
use self::blas::default::Default;
use self::blas::matrix::Matrix;
use self::blas::matrix_vector::ops::{
    Gemv,
};
use self::blas::vector::ops::Copy;

pub struct Mat<T> {
    rows: uint,
    cols: uint,
    data: Vec<T>,
}

impl<T> Mat<T> {
    #[inline]
    pub fn new() -> Mat<T> {
        let _data: Vec<T> = Vec::new();
        Mat { rows: 0, cols: 0, data: _data }
    }

    #[inline]
    pub fn from_vec(rows: uint, cols: uint, vec: Vec<T>) -> Mat<T> {
        Mat { rows: rows, cols: cols, data: vec }
    }

    #[inline]
    pub fn zero(rows: uint, cols: uint) -> Mat<T> {
        let mut _data: Vec<T> = Vec::with_capacity(rows * cols);
        unsafe { _data.set_len(rows * cols); }
        Mat { rows: rows, cols: cols, data: _data}
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    #[inline]
    pub fn rows(&self) -> uint {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> uint {
        self.cols
    }

    #[inline]
    pub unsafe fn set_rows(&mut self, rows: uint) {
        self.rows = rows;
    }

    #[inline]
    pub unsafe fn set_cols(&mut self, cols: uint) {
        self.cols = cols;
    }

    #[inline]
    pub unsafe fn as_slice<'a>(&'a self) -> &'a [T] {
        self.data.as_slice()
    }

    #[inline]
    pub unsafe fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        self.data.as_mut_slice()
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

impl<T> Index<uint, [T]> for Mat<T> {
    fn index<'a>(&'a self, index: &uint) -> &'a [T] {
        unsafe {
            let ptr = self.data.as_slice().as_ptr().offset((*index * self.cols) as int);
            mem::transmute(Slice { data: ptr, len: self.cols })
        }
    }
}

impl<T: fmt::Show> fmt::Show for Mat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in range(0u, self.rows) {
            match writeln!(f, "{}", &self[i]) {
                Ok(_) => (),
                x => return x,
            }
        }

        Ok(())
    }
}

impl<T: Default + Gemv> Mul<Vec<T>, Vec<T>> for Mat<T> {
    fn mul(&self, x: &Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(self.rows);
        let one: T = Default::one();
        let zero: T = Default::zero();

        Gemv::gemv(&one, self, x, &zero, &mut result);
        unsafe { result.set_len(self.rows); }

        result
    }
}

#[macro_export]
macro_rules! mat(
    ($([$($e: expr),+]),*) => ({
        // leading _ to allow empty construction without a warning.
        let mut _temp = Mat::new();
        let mut rows = 0u;
        let mut _cols;
        $(
            rows += 1;
            _cols = 0u;
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
)

#[cfg(test)]
mod tests {
    extern crate test;

    use mat::Mat;

    #[test]
    fn index() {
        let m = mat![[1f32,2f32],[3f32,4f32]];

        assert_eq!(1.0, m[0][0]);
        assert_eq!(2.0, m[0][1]);
        assert_eq!(3.0, m[1][0]);
        assert_eq!(4.0, m[1][1]);
    }

    #[test]
    fn mul_vec() {
        let a = mat![
            [1f32, -2f32],
            [2f32, -4f32]
        ];
        let x = vec![2f32, 1f32];

        assert_eq!(a * x, vec![0f32, 0f32]);
    }
}