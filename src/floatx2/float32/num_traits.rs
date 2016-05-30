
use super::rf32x2;
use num_traits::identities::{One,Zero};


impl Zero for rf32x2 {
	fn zero() -> Self {
		rf32x2 {
			v0: 0f32,
			v1: 0f32
		}
	}
	fn is_zero(&self) -> bool {
		self.v0.is_zero() && self.v1.is_zero()
	}
}

impl One for rf32x2 {
	fn one() -> Self {
		rf32x2 {
			v0: 1f32,
			v1: 1f32
		}
	}
}