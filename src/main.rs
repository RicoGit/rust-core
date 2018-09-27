#![feature(alloc)]
#![feature(allocator_api)]

extern crate core;
extern crate alloc;

mod stack;
mod alloc_example;

fn main() {

//    stack::demo_simple_stack();

    alloc_example::demo_str_passing("test string")

}
