

pub trait SseArth {
	fn addps(self, rhs:Self) -> Self;
	fn subps(self, rhs:Self) -> Self;
	fn mulps(self, rhs:Self) -> Self;
	fn divps(self, rhs:Self) -> Self;
	fn sqrtps(self) -> Self;
	fn maxps(self, rhs:Self) -> Self;
	fn minps(self, rhs:Self) -> Self;
}