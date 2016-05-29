
use std::default::Default;
use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use simd::SseArth;
use std::fmt;

extern "C" {
    fn _add_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _sub_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _mul_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _div_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _max_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _min_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _sqrt_f32x2(a: rf32x2) -> rf32x2;
}

#[derive(Debug,Clone,Copy,PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rf32x2 {
    pub v0: f32,
    pub v1: f32,
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

impl SseArth for rf32x2 {
	#[inline]
	fn addps(self, rhs: Self) -> Self {
		unsafe {
			_add_f32x2(self,rhs)
		}
	}
	#[inline]
	fn subps(self, rhs: Self) -> Self {
		unsafe {
			_sub_f32x2(self,rhs)
		}
	}
	#[inline]
	fn mulps(self, rhs: Self) -> Self {
		unsafe {
			_mul_f32x2(self,rhs)
		}
	}
	#[inline]
	fn divps(self, rhs: Self) -> Self {
		unsafe {
			_div_f32x2(self,rhs)
		}
	}
	#[inline]
	fn maxps(self, rhs: Self) -> Self {
		unsafe {
			_max_f32x2(self,rhs)
		}
	}
	#[inline]
	fn minps(self, rhs: Self) -> Self {
		unsafe {
			_min_f32x2(self,rhs)
		}
	}
	#[inline]
	fn sqrtps(self) -> Self {
		unsafe {
			_sqrt_f32x2(self)
		}
	}
}
