#[macro_use]
extern crate rfloat;
extern crate frounding;
extern crate num_traits;

use rfloat::rf32x2;
use rfloat::rf32x4;
use rfloat::rf64x2;

fn main() {
	let mut rs = frounding::RoundingState::new();
	rs.upward();
    let mut b = rf32x2!(1f32);
    let mut c = rf32x4!(1f32);
    let mut d = rf64x2!(1f64);
    d /= 3.0;
    b /= 3f32;
    c /= 6f32;
    println!("{} / {} / {}", b, c, d);
}
