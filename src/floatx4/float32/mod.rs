
use std::default::Default;
use std::fmt;

mod ops_traits;
mod simd_traits;
mod num_traits;

#[derive(Debug,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
/// f32x4 simd data type.
pub struct rf32x4 {
    pub v0: f32,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
}

#[macro_export]
/// Create new rf32x4.
/// '''
/// 
macro_rules! rf32x4 {
    ($v:expr) => {
        rf32x4 {
            v0: $v,
            v1: $v,
            v2: $v,
            v3: $v,
        }
    };
    ($v0:expr,$v1:expr,$v2:expr,$v3:expr) => {
    	rf32x4 {
    		v0: $v0,
    		v1: $v1,
    		v2: $v2,
    		v3: $v3,
    	}
    };
}

impl Default for rf32x4 {
    fn default() -> rf32x4 {
        rf32x4!(0f32,0f32,0f32,0f32)
    }
}

impl fmt::Display for rf32x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{}", self.v0, self.v1, self.v2, self.v3)
    }
}