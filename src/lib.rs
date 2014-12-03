// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![feature(macro_rules)]

pub mod vec;
pub mod mat;

pub enum Trans<Tensor> {
	T(Tensor),
	H(Tensor),
}

impl<T> Trans<T> {
    pub fn into_inner(&self) -> &T {
        match self {
            &Trans::T(ref t) => t,
            &Trans::H(ref t) => t,
        }
    }
}
