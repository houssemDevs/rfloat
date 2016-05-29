
extern crate gcc;

fn main(){
	gcc::compile_library("libsimd.a", &["csrc/sse.c"]);
}
