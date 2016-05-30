
use super::rf64x2;
use float::rf64;
use simd::Sse2Arth;
use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};

impl Add for rf64x2 {
	type Output = rf64x2;
	fn add(self, rhs: Self) -> Self {
		self.addpd(rhs)
	}
}

impl Sub for rf64x2 {
	type Output = rf64x2;
	fn sub(self, rhs: Self) -> Self {
		self.subpd(rhs)
	}
}

impl Mul for rf64x2 {
	type Output = rf64x2;
	fn mul(self, rhs: Self) -> Self {
		self.mulpd(rhs)
	}
}

impl Div for rf64x2 {
	type Output = rf64x2;
	fn div(self, rhs: Self) -> Self {
		self.divpd(rhs)
	}
}

impl AddAssign<rf64x2> for rf64x2 {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = self.addpd(rhs);
	}
}

impl AddAssign<rf64> for rf64x2 {
	#[inline]
	fn add_assign(&mut self, rhs: rf64) {
		let a = rf64x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.addpd(a);
	}
}

impl AddAssign<f64> for rf64x2 {
	#[inline]
	fn add_assign(&mut self, rhs: f64) {
		let a = rf64x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.addpd(a);
	}
}

impl SubAssign<rf64x2> for rf64x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.subpd(rhs);
	}
}

impl SubAssign<rf64> for rf64x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: rf64) {
		let a = rf64x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.subpd(a);
	}
}

impl SubAssign<f64> for rf64x2 {
	#[inline]
	fn sub_assign(&mut self, rhs: f64) {
		let a = rf64x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.subpd(a);
	}
}

impl MulAssign<rf64x2> for rf64x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.mulpd(rhs);
	}
}

impl MulAssign<rf64> for rf64x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: rf64) {
		let a = rf64x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.mulpd(a);
	}
}

impl MulAssign<f64> for rf64x2 {
	#[inline]
	fn mul_assign(&mut self, rhs: f64) {
		let a = rf64x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.mulpd(a);
	}
}

impl DivAssign<rf64x2> for rf64x2 {
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = self.divpd(rhs);
	}
}

impl DivAssign<rf64> for rf64x2 {
	#[inline]
	fn div_assign(&mut self, rhs: rf64) {
		let a = rf64x2{
			v0: rhs.0,
			v1: rhs.0
		};
		*self = self.divpd(a);
	}
}

impl DivAssign<f64> for rf64x2 {
	#[inline]
	fn div_assign(&mut self, rhs: f64) {
		let a = rf64x2{
			v0: rhs,
			v1: rhs
		};
		*self = self.divpd(a);
	}
}