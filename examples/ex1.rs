extern crate rfloat;
extern crate frounding;
extern crate num_traits;


use num_traits::Float;
use rfloat::rf64;
use frounding::RoundingState;

fn main() {
    let mut rstate = RoundingState::new();
    let mut x = rf64(1.0);
    let y = rf64(3.0);
    rstate.upward();
    println!("{:.100}", x / y);
    rstate.downward();
    println!("{:.100}", x / y);
    println!("{}", x == y);
    println!("{}", x.sin());
    x += 15.0;
    println!("{}", x);
}
