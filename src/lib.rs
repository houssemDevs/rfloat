#![warn(missing_docs)]
#![feature(float_extras)]

//!rfloat is a wrap of floating primitive types of rust. Its purpose is to avoid LLVM
//!Optimizations that ignore the current CPU rounding mode.

extern crate num_traits;


mod float;
pub use float::{rf32,rf64};
