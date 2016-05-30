
use std::default::Default;
use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use std::fmt;
use simd::SseArth;
use float::rf32;

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
/// f32x2 simd data type.
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

impl Add for rf32x2 {
	type Output = rf32x2;
	#[inline]
	fn add(self, rhs: Self) -> Self {
		self.addps(rhs)
	}
}

impl Sub for rf32x2 {
	type Output = rf32x2;
	#[inline]
	fn sub(self, rhs: Self) -> Self {
		self.subps(rhs)
	}
}

impl Mul for rf32x2 {
	type Output = rf32x2;
	#[inline]
	fn mul(self, rhs: Self) -> Self {
		self.mulps(rhs)
	}
}

impl Div for rf32x2 {
	type Output = rf32x2;
	#[inline]
	fn div(self, rhs: Self) -> Self {
		self.divps(rhs)
	}
}

impl AddAssign<rf32x2> for rf32x2 {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = self.addps(rhs);
	}
}

impl AddAssign<rf32> for rf32x2 {
	#[inline]
	fn add_assign(&mut self, rhs: rf32) {
		let a = rf32x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.addps(a);
	}
}

impl AddAssign<f32> for rf32x2 {
	#[inline]
	fn add_assign(&mut self, rhs: f32) {
		let a = rf32x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.addps(a);
	}
}

impl SubAssign<rf32x2> for rf32x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.subps(rhs);
	}
}

impl SubAssign<rf32> for rf32x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: rf32) {
		let a = rf32x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.subps(a);
	}
}

impl SubAssign<f32> for rf32x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: f32) {
		let a = rf32x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.subps(a);
	}
}

impl MulAssign<rf32x2> for rf32x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.mulps(rhs);
	}
}

impl MulAssign<rf32> for rf32x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: rf32) {
		let a = rf32x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.mulps(a);
	}
}

impl MulAssign<f32> for rf32x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: f32) {
		let a = rf32x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.mulps(a);
	}
}

impl DivAssign<rf32x2> for rf32x2 {
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = self.divps(rhs);
	}
}

impl DivAssign<rf32> for rf32x2 {
	#[inline]
	fn div_assign(&mut self, rhs: rf32) {
		let a = rf32x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.divps(a);
	}
}

impl DivAssign<f32> for rf32x2 {
	#[inline]
	fn div_assign(&mut self, rhs: f32) {
		let a = rf32x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.divps(a);
	}
}