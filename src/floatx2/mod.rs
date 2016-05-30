#[cfg(sse)]
mod float32;
#[cfg(sse)]
pub use self::float32::rf32x2;

#[cfg(sse2)]
mod float64;
#[cfg(sse2)]
pub use self::float64::rf64x2;
