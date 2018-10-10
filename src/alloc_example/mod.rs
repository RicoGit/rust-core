#![allow(dead_code)]

use core::ptr;
use std::str;

pub mod alloc_example;

/// Copies string to Heap with alloc API and read it back from Heap with alloc API
pub fn demo_str_passing(string: &str) {
    let str_len = string.len();
    match alloc_example::alloc(str_len) {
        Some(ptr) => {
            println!(
                "Allocated {} bytes with aligning by address {:?}",
                &str_len, ptr
            );

            unsafe {
                println!("Write string to memory by address={:?}", ptr.as_ptr());
                for (idx, byte) in string.as_bytes().iter().enumerate() {
                    ptr::write(ptr.as_ptr().offset(idx as isize), *byte)
                }

                let string = String::from_raw_parts(ptr.as_ptr(), str_len, str_len);
                println!("Read string from memory: '{}'", string);
            }
        }
        None => println!("Unable to allocate {} bytes", &str_len),
    }
}
