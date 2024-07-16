// src/main.rs
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[link(name = "c_lib")]
extern "C" {
    fn c_function(input: *const c_char) -> c_int;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = CString::new("Hello from Rust!")?;

    unsafe {
        match c_function(input.as_ptr()) {
            0 => println!("C function executed successfully"),
            error_code => println!("C function returned error code: {}", error_code),
        }
    }

    Ok(())
}
