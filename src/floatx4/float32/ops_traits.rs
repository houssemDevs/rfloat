
use simd::SseArth;
use super::rf32x4;
use float::rf32;
use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};


impl Add for rf32x4 {
	type Output = rf32x4;
	#[inline]
	fn add(self, rhs: Self) -> Self {
		self.addps(rhs)
	}
}

impl Sub for rf32x4 {
	type Output = rf32x4;
	#[inline]
	fn sub(self, rhs: Self) -> Self {
		self.subps(rhs)
	}
}

impl Mul for rf32x4 {
	type Output = rf32x4;
	#[inline]
	fn mul(self, rhs: Self) -> Self {
		self.mulps(rhs)
	}
}

impl Div for rf32x4 {
	type Output = rf32x4;
	#[inline]
	fn div(self, rhs: Self) -> Self {
		self.divps(rhs)
	}
}

impl AddAssign<rf32x4> for rf32x4 {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = self.addps(rhs);
	}
}

impl AddAssign<rf32> for rf32x4 {
	#[inline]
	fn add_assign(&mut self, rhs: rf32) {
		let a = rf32x4{
			v0: rhs.0,
			v1: rhs.0,
			v2: rhs.0,
			v3: rhs.0
		};
		*self = self.addps(a);
	}
}

impl AddAssign<f32> for rf32x4 {
	#[inline]
	fn add_assign(&mut self, rhs: f32) {
		let a = rf32x4{
			v0: rhs,
			v1: rhs,
			v2: rhs,
			v3: rhs
		};
		*self = self.addps(a);
	}
}

impl SubAssign<rf32x4> for rf32x4 {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.subps(rhs);
	}
}

impl SubAssign<rf32> for rf32x4 {
	#[inline]
	fn sub_assign(&mut self, rhs: rf32) {
		let a = rf32x4{
			v0: rhs.0,
			v1: rhs.0,
			v2: rhs.0,
			v3: rhs.0
		};
		*self = self.subps(a);
	}
}

impl SubAssign<f32> for rf32x4 {
	#[inline]
	fn sub_assign(&mut self, rhs: f32) {
		let a = rf32x4{
			v0: rhs,
			v1: rhs,
			v2: rhs,
			v3: rhs
		};
		*self = self.subps(a);
	}
}

impl MulAssign<rf32x4> for rf32x4 {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.mulps(rhs);
	}
}

impl MulAssign<rf32> for rf32x4 {
	#[inline]
	fn mul_assign(&mut self, rhs: rf32) {
		let a = rf32x4{
			v0: rhs.0,
			v1: rhs.0,
			v2: rhs.0,
			v3: rhs.0
		};
		*self = self.mulps(a);
	}
}

impl MulAssign<f32> for rf32x4 {
	#[inline]
	fn mul_assign(&mut self, rhs: f32) {
		let a = rf32x4{
			v0: rhs,
			v1: rhs,
			v2: rhs,
			v3: rhs
		};
		*self = self.mulps(a);
	}
}

impl DivAssign<rf32x4> for rf32x4 {
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = self.divps(rhs);
	}
}

impl DivAssign<rf32> for rf32x4 {
	#[inline]
	fn div_assign(&mut self, rhs: rf32) {
		let a = rf32x4{
			v0: rhs.0,
			v1: rhs.0,
			v2: rhs.0,
			v3: rhs.0
		};
		*self = self.divps(a);
	}
}

impl DivAssign<f32> for rf32x4 {
	#[inline]
	fn div_assign(&mut self, rhs: f32) {
		let a = rf32x4{
			v0: rhs,
			v1: rhs,
			v2: rhs,
			v3: rhs
		};
		*self = self.divps(a);
	}
}