extern crate rfloat;
extern crate frounding;
extern crate num_traits;


use num_traits::{Float,Num};
use rfloat::{rf64,rf32};
use frounding::RoundingState;

fn main() {    
    let mut x = rf64(1.0);
    let y = rf64(3.0);
    {
    	let mut rstate = RoundingState::new();
    	rstate.upward();
    	println!("{:.17}", x / y);
    	rstate.downward();
    	println!("{:.17}", x / y);
   	 	println!("{}", x == y);
    	println!("{}", x.sin());
    }
    let tt = rf64::from_str_radix("23.4E2", 10);
    println!("{}", tt.unwrap());
    x += 15.0;
    println!("{}", x);
}
