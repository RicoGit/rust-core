#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate core;

mod algorithms;
mod alloc_example;

fn main() {
    algorithms::stack::demo_simple_stack();

    alloc_example::demo_str_passing("test string")
}
