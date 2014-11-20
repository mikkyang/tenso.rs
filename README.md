# Tenso.rs

A (hopefully) fast math library for Rust, using BLAS bindings for computation.
LAPACK bindings should be added in the near future.

As a side note, I have no idea if the name of this library is mathematically
correct.

# Dependencies

* Rust: Tenso.rs is built on the nightly builds of the Rust repository.
* BLAS: By default, Tenso.rs links against the BLAS library on your system,
using the `-lblas` flag.
