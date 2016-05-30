use super::rf32x4;
use num_traits::identities::{One,Zero};


impl Zero for rf32x4 {
	fn zero() -> Self {
		rf32x4 {
			v0: 0f32,
			v1: 0f32,
			v2: 0f32,
			v3: 0f32
		}
	}
	fn is_zero(&self) -> bool {
		self.v0.is_zero() && self.v1.is_zero() && self.v2.is_zero() && self.v3.is_zero()
	}
}

impl One for rf32x4 {
	fn one() -> Self {
		rf32x4 {
			v0: 1f32,
			v1: 1f32,
			v2: 1f32,
			v3: 1f32
		}
	}
}