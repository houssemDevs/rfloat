
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::num::FpCategory;
use std::fmt;
use num_traits::{ParseFloatError, Num, Float};
use num_traits::identities::{Zero, One};
use num_traits::cast::{ToPrimitive, NumCast};


#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
#[allow(non_camel_case_types)]
/// Wrap the f32 primitive.
pub struct rf32(pub f32);

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
#[allow(non_camel_case_types)]
/// Wrap the f64 primitive.
pub struct rf64(pub f64);

macro_rules! impl_trait_ops {
    ($($t:ident),*) => {   	
    	$(
    	 impl Add for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn add(self, rhs: Self) -> Self::Output {
    	 		$t(self.0 + rhs.0)
    	 	}
    	 }
    	 impl Sub for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn sub(self, rhs: Self) -> Self::Output {
    	 		$t(self.0 - rhs.0)
    	 	}
    	 }
    	 impl Mul for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn mul(self, rhs: Self) -> Self::Output {
    	 		$t(self.0 * rhs.0)
    	 	}
    	 }
    	 impl Div for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn div(self, rhs: Self) -> Self::Output {
    	 		$t(self.0 / rhs.0)
    	 	}
    	 }
    	 impl Rem for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn rem(self, rhs: Self) -> Self::Output {
    	 		$t(self.0 % rhs.0)
    	 	}
    	 }
    	 impl Neg for $t {
    	 	type Output = Self;
    	 	#[inline(never)]
    	 	fn neg(self) -> Self::Output {
    	 		$t(-self.0)
    	 	}
    	 }
    	)*
    }
}

macro_rules! impl_trait_ops_assign {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl AddAssign<$t> for $t {
    			#[inline(never)]
    			fn add_assign(&mut self, rhs: $t) {
    				self.0 = self.0 + rhs.0;
    			}
    		}
    		impl AddAssign<$st> for $t {
    			#[inline(never)]
    			fn add_assign(&mut self, rhs: $st) {
    				self.0 = self.0 + rhs;
    			}
    		}
    		impl SubAssign<$t> for $t {
    			#[inline(never)]
    			fn sub_assign(&mut self, rhs: $t) {
    				self.0 = self.0 - rhs.0;
    			}
    		}
    		impl SubAssign<$st> for $t {
    			#[inline(never)]
    			fn sub_assign(&mut self, rhs: $st) {
    				self.0 = self.0 - rhs;
    			}
    		}
    		impl MulAssign<$t> for $t {
    			#[inline(never)]
    			fn mul_assign(&mut self, rhs: $t) {
    				self.0 = self.0 * rhs.0;
    			}
    		}
    		impl MulAssign<$st> for $t {
    			#[inline(never)]
    			fn mul_assign(&mut self, rhs: $st) {
    				self.0 = self.0 * rhs;
    			}
    		}
    		impl DivAssign<$t> for $t {
    			#[inline(never)]
    			fn div_assign(&mut self, rhs: $t) {
    				self.0 = self.0 / rhs.0;
    			}
    		}
    		impl DivAssign<$st> for $t {
    			#[inline(never)]
    			fn div_assign(&mut self, rhs: $st) {
    				self.0 = self.0 / rhs;
    			}
    		}
    		impl RemAssign<$t> for $t {
    			#[inline(never)]
    			fn rem_assign(&mut self, rhs: $t) {
    				self.0 = self.0 % rhs.0;
    			}
    		}
    		impl RemAssign<$st> for $t {
    			#[inline(never)]
    			fn rem_assign(&mut self, rhs: $st) {
    				self.0 = self.0 % rhs;
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_identities {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl Zero for $t {
    			#[inline]
    			fn zero() -> Self {
    				$t($st::zero())
    			}
    			#[inline]
    			fn is_zero(&self) -> bool {
    				self.0.is_zero()
    			}
    		}
    		impl One for $t {
    			fn one() -> Self {
    				$t($st::one())
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_num {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl Num for $t {
    			type FromStrRadixErr = ParseFloatError;
    			#[inline]
    			fn from_str_radix(src: &str, radix: u32) -> Result<Self,Self::FromStrRadixErr>{
    				match $st::from_str_radix(src,radix) {
    					Ok(n) => Ok($t(n)),
						Err(e) => Err(e),
    				}
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_toPrimitive {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl ToPrimitive for $t {
    			#[inline]
    			fn to_i64(&self) -> Option<i64> {
    				self.0.to_i64()
    			}
    			#[inline]
    			fn to_u64(&self) -> Option<u64> {
    				self.0.to_u64()
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_numcast {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl NumCast for $t {
    			fn from<T: ToPrimitive>(n: T) -> Option<Self> {
    				match n.$st() {
    					Some(num) => Some($t(num)),
    					_ => None 
    				}
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_float {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl Float for $t {
    			fn nan() -> Self { $t($st::nan()) }
    			fn infinity() -> Self { $t($st::infinity()) }
    			fn neg_infinity() -> Self { $t($st::neg_infinity()) }
    			fn neg_zero() -> Self { $t($st::neg_zero()) }
			    fn min_value() -> Self { $t($st::min_value()) }
    			fn min_positive_value() -> Self { $t($st::min_positive_value()) }
    			fn max_value() -> Self { $t($st::max_value()) }
    			fn is_nan(self) -> bool { self.0.is_nan() }
    			fn is_infinite(self) -> bool { self.0.is_infinite() }
    			fn is_finite(self) -> bool { self.0.is_finite() }
    			fn is_normal(self) -> bool { self.0.is_normal() }
    			fn classify(self) -> FpCategory { self.0.classify() }
			    fn floor(self) -> Self { $t(self.0.floor()) }
    			fn ceil(self) -> Self { $t(self.0.ceil()) }
    			fn round(self) -> Self { $t(self.0.round()) }
    			fn trunc(self) -> Self { $t(self.0.trunc()) }
    			fn fract(self) -> Self { $t(self.0.fract()) }
    			fn abs(self) -> Self { $t(self.0.abs()) }
    			fn signum(self) -> Self { $t(self.0.signum()) }
    			fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }
    			fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }
    			fn mul_add(self, a: Self, b: Self) -> Self { $t($st::mul_add(self.0,a.0,b.0)) }
    			fn recip(self) -> Self { $t(self.0.recip()) }
    			fn powi(self, n: i32) -> Self { $t(self.0.powi(n)) }
    			fn powf(self, n: Self) -> Self { $t(self.0.powf(n.0)) }
    			fn sqrt(self) -> Self { $t(self.0.sqrt()) }
    			fn exp(self) -> Self { $t(self.0.exp()) }
    			fn exp2(self) -> Self { $t(self.0.exp2()) }
    			fn ln(self) -> Self { $t(self.0.ln()) }
    			fn log(self, base: Self) -> Self { $t(self.0.log(base.0)) }
    			fn log2(self) -> Self { $t(self.0.log2()) }
    			fn log10(self) -> Self { $t(self.0.log10()) }
    			fn max(self, other: Self) -> Self { $t(self.0.max(other.0)) }
    			fn min(self, other: Self) -> Self { $t(self.0.min(other.0)) }
    			fn abs_sub(self, other: Self) -> Self { $t(self.0.abs_sub(other.0)) }
    			fn cbrt(self) -> Self { $t(self.0.cbrt()) }
    			fn hypot(self, other: Self) -> Self { $t(self.0.hypot(other.0)) }
    			fn sin(self) -> Self { $t(self.0.sin()) }
    			fn cos(self) -> Self { $t(self.0.cos()) }
    			fn tan(self) -> Self { $t(self.0.tan()) }
    			fn asin(self) -> Self { $t(self.0.asin()) }
    			fn acos(self) -> Self { $t(self.0.acos()) }
    			fn atan(self) -> Self { $t(self.0.atan()) }
    			fn atan2(self, other: Self) -> Self { $t(self.0.atan2(other.0)) }
    			fn sin_cos(self) -> (Self, Self) { 
    				let (s,c) = self.0.sin_cos();
    				($t(s),$t(c))
    			}
    			fn exp_m1(self) -> Self { $t(self.0.exp_m1()) }
    			fn ln_1p(self) -> Self { $t(self.0.ln_1p()) }
    			fn sinh(self) -> Self { $t(self.0.sinh()) }
    			fn cosh(self) -> Self { $t(self.0.cosh()) }
    			fn tanh(self) -> Self { $t(self.0.tanh()) }
    			fn asinh(self) -> Self { $t(self.0.asinh()) }
    			fn acosh(self) -> Self { $t(self.0.acosh()) }
    			fn atanh(self) -> Self { $t(self.0.atanh()) }
    			fn integer_decode(self) -> (u64, i16, i8) {
    				self.0.integer_decode()
    			}
    		}
    	)*
    }
}

macro_rules! impl_trait_display {
    ($($t:ident $st:ident),*) => {
    	$(
    		impl fmt::Display for $t {
    			#[inline]
    			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    				$st::fmt(&self.0, f)
    			}
    		}
    	)*
    }
}

impl_trait_ops!(rf32, rf64); // Implement +,-,*,/,%,-
impl_trait_ops_assign!(rf32 f32, rf64 f64); // Implement +=,-=,*=,/=,%=
impl_trait_identities!(rf32 f32,rf64 f64); // Implement Zero,One
impl_trait_num!(rf32 f32,rf64 f64); // Implement Num trait
impl_trait_toPrimitive!(rf32 f32,rf64 f64); // Implement ToPrimitive trait
impl_trait_numcast!(rf32 to_f32,rf64 to_f64); // Implement NumCast trait
impl_trait_float!(rf32 f32,rf64 f64); // Implement Float trait
impl_trait_display!(rf32 f32,rf64 f64); // Implement Display trait
