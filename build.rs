
extern crate gcc;
extern crate cupid;

fn main() {
	let mut cfiles: Vec<&str> = Vec::new();
    if let Some(cpu_info) = cupid::master() {
        if cpu_info.sse() {
        	cfiles.push("csrc/sse.c");
        	println!("cargo:rustc-cfg=sse");
        }
        if cpu_info.sse2() {
        	cfiles.push("csrc/sse2.c");
        	println!("cargo:rustc-cfg=ss2");
        }
    }
    gcc::compile_library("libsimd.a", &cfiles);
}
