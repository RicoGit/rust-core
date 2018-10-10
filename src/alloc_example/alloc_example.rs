use core::alloc::Layout;
use std::alloc::{Alloc, Global};
use std::mem;
use std::ptr::NonNull;

#[allow(dead_code)]
pub fn alloc(size: usize) -> Option<NonNull<u8>> {
    unsafe {
        Layout::from_size_align(size, mem::align_of::<u8>())
            .ok()
            .and_then(|l| Global.alloc(l).ok())
    }
}

#[allow(dead_code)]
pub fn dealloc(ptr: NonNull<u8>, size: usize) {
    unsafe {
        Layout::from_size_align(size, mem::align_of::<u8>())
            .ok()
            .map(|l| Global.dealloc(ptr, l));
    }
}
