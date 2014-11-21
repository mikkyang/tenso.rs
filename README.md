# Tensors

A (hopefully) fast math library for Rust, using BLAS bindings for computation.
LAPACK bindings should be added in the near future.

As a side note, I have no idea if the name of this library is mathematically
correct.

```toml
# Cargo.toml
[dependencies.tensors]

git = "https://github.com/mikkyang/tenso.rs.git"
```

# Dependencies

* Rust: Tensors is built on the nightly builds of the Rust repository.
* BLAS: By default, Tensors links against the BLAS library on your system,
using the `-lblas` flag.

# Overview

Tensors provides high level wrappers for low level linear algebra operations.

Scalar types are the built-in ones: `f32`, `f64`, `Complex32`, `Complex64`.

Container types are Tensors specific: `Vec`, `Mat`.

There are also types to add attributes to these containers for transpose or Hermitian transpose operations.

For example, calculating dot product:

```rust
extern crate tensors;

use tensors::vec::Vec as TVec;
use tensors::vec::TransVec::T;

fn main() {
    let x = TVec::from_vec(vec![1f32, 2f32]);
    let y = TVec::from_vec(vec![-3f32, 1f32]);

    assert_eq!(T(x) * y, -1f32);
}

```
