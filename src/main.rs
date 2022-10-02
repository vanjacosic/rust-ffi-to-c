extern crate core;
use core::ffi::c_int;

extern "C" {
    fn multiply(a: c_int, b: c_int) -> c_int;
}

fn main() {
    println!("[Rust] Hello from Rust! 🦀");

    unsafe {
        println!("[Rust] Calling function in C..");

        let result = multiply(5000, 5);

        println!("[Rust] Result: {}", result);
    }
}
