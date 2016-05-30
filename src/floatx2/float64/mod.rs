
use std::default::Default;
use std::fmt;

mod num_traits;
mod ops_traits;
mod simd_traits;


#[derive(Debug,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rf64x2 {
	pub v0: f64,
	pub v1: f64
}

#[macro_export]
/// Create new f64x2.
macro_rules! rf64x2 {
    ($v:expr) => {
    	rf64x2 {
    		v0: $v,
    		v1: $v
    	}
    };
    ($v0:expr,$v1:expr) => {
    	rf64x2 {
    		v0: $v0,
    		v1: $v1
    	}
    };
}

impl Default for rf64x2 {
	fn default() -> rf64x2 {
		rf64x2!(0f64)
	}
}

impl fmt::Display for rf64x2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.v0, self.v1)
	}
}