
use super::rf64x2;
use simd::Sse2Arth;

extern "C" {
    fn _add_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _sub_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _mul_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _div_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _max_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _min_f64x2(a: rf64x2, b: rf64x2) -> rf64x2;
    fn _sqrt_f64x2(a: rf64x2) -> rf64x2;
}

impl Sse2Arth for rf64x2 {
	#[inline]
	fn addpd(self, rhs: Self) -> Self {
		unsafe {
			_add_f64x2(self, rhs)
		}
	}
	#[inline]
	fn subpd(self, rhs: Self) -> Self {
		unsafe {
			_sub_f64x2(self, rhs)
		}
	}
	#[inline]
	fn mulpd(self, rhs: Self) -> Self {
		unsafe {
			_mul_f64x2(self, rhs)
		}
	}
	#[inline]
	fn divpd(self, rhs: Self) -> Self {
		unsafe {
			_div_f64x2(self, rhs)
		}
	}
	#[inline]
	fn maxpd(self, rhs: Self) -> Self {
		unsafe {
			_max_f64x2(self, rhs)
		}
	}
	#[inline]
	fn minpd(self, rhs: Self) -> Self {
		unsafe {
			_min_f64x2(self, rhs)
		}
	}
	#[inline]
	fn sqrtpd(self) -> Self {
		unsafe {
			_sqrt_f64x2(self)
		}
	}
}