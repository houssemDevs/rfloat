#[cfg(sse)]
mod float32;
#[cfg(sse)]
pub use self::float32::rf32x4;