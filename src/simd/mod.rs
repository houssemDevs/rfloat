

pub trait SseArth {
	fn addps(self, rhs:Self) -> Self;
	fn subps(self, rhs:Self) -> Self;
	fn mulps(self, rhs:Self) -> Self;
	fn divps(self, rhs:Self) -> Self;
	fn sqrtps(self) -> Self;
	fn maxps(self, rhs:Self) -> Self;
	fn minps(self, rhs:Self) -> Self;
}


pub trait Sse2Arth {
	fn addpd(self, rhs:Self) -> Self;
	fn subpd(self, rhs:Self) -> Self;
	fn mulpd(self, rhs:Self) -> Self;
	fn divpd(self, rhs:Self) -> Self;
	fn sqrtpd(self) -> Self;
	fn maxpd(self, rhs:Self) -> Self;
	fn minpd(self, rhs:Self) -> Self;
}