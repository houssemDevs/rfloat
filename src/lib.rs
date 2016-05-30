//#![warn(missing_docs)]
#![feature(float_extras)]

//!rfloat is a wrap of floating primitive types of rust. Its purpose is to avoid LLVM
//!Optimizations that ignore the current CPU rounding mode.

extern crate num_traits;

#[allow(dead_code)]
mod simd;

mod float;
pub use float::{rf32,rf64};

#[allow(dead_code)]
mod floatx2;
#[cfg(sse)]
pub use floatx2::rf32x2;
#[cfg(sse2)]
pub use floatx2::rf64x2;

#[allow(dead_code)]
mod floatx4;
#[cfg(sse)]
pub use floatx4::rf32x4;