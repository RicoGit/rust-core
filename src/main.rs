#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate core;
#[macro_use]
extern crate error_chain;

mod algorithms;
mod alloc_example;
mod errors_chain_example;

fn main() {
    algorithms::stack::demo_simple_stack();
    algorithms::heap::demo_vector_based_heap();

    alloc_example::demo_str_passing("test string");

    if let Err(ref e) = errors_chain_example::run() {
        use error_chain::ChainedError;
        println!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}
