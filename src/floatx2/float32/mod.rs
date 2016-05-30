

use std::default::Default;
use std::fmt;

mod num_traits;
mod ops_traits;
mod simd_traits;

#[derive(Debug,Clone,Copy,PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
/// f32x2 simd data type.
pub struct rf32x2 {
    pub v0: f32,
    pub v1: f32,
}

#[macro_export]
/// Create new rf32x2
macro_rules! rf32x2 {
	($v:expr) => {
		rf32x2 {
			v0: $v,
			v1: $v		
		}
	};
	($v0:expr,$v1:expr) => {
		rf32x2 {
			v0: $v0,
			v1: $v1		
		}
	};
}

impl Default for rf32x2 {
    fn default() -> rf32x2 {
        rf32x2 {
            v0: 0f32,
            v1: 0f32,
        }
    }
}

impl fmt::Display for rf32x2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.v0, self.v1)
	}
}






