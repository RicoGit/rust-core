#![feature(allocator_api)]
#![feature(slice_ptr_get)]

use core::alloc::Layout;
use std::alloc::{Allocator, Global};
use std::error::Error;
use std::mem;
use std::ptr::NonNull;

#[allow(dead_code)]
pub fn alloc(size: usize) -> Result<NonNull<[u8]>, Box<dyn Error>> {
    let layout = Layout::from_size_align(size, mem::align_of::<u8>())?;
    Ok(Global.allocate(layout)?)
}

#[allow(dead_code)]
pub fn dealloc(ptr: NonNull<u8>, size: usize) -> Result<(), Box<dyn Error>> {
    unsafe {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>())?;
        Ok(Global.deallocate(ptr, layout))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn str_passing_test() {
        // Copies string to Heap with alloc API and read it back from Heap with alloc API

        let input_string = "Some test string!/n ...";
        let str_len = input_string.len();
        match alloc(str_len) {
            Ok(ptr) => {
                println!(
                    "Allocated {} bytes with aligning by address {:?}",
                    &str_len, ptr
                );
                unsafe {
                    println!("Write string to memory by address={:?}", ptr.as_ptr());
                    for (idx, byte) in input_string.as_bytes().iter().enumerate() {
                        ptr.as_mut_ptr().add(idx).write(*byte)
                    }

                    let result_string = String::from_raw_parts(ptr.as_mut_ptr(), str_len, str_len);
                    println!("Read string from memory: '{}'", result_string);
                    assert_eq!(input_string, result_string)
                }
            }
            Err(err) => println!("Unable to allocate {} bytes, err: {}", &str_len, err),
        }
    }
}
