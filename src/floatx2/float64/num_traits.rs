
use super::rf64x2;
use num_traits::identities::{Zero,One};


impl Zero for rf64x2 {
	fn zero() -> Self {
		rf64x2 {
			v0: 0f64,
			v1: 0f64
		}
	}
	fn is_zero(&self) -> bool {
		self.v0.is_zero() && self.v1.is_zero()
	}
}

impl One for rf64x2 {
	fn one() -> Self {
		rf64x2 {
			v0: 1f64,
			v1: 1f64
		}
	}
}