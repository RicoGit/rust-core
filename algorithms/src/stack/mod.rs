#![allow(dead_code)]

pub mod naive_stack;

pub fn demo_simple_stack() {
    let mut stack = naive_stack::SimpleStack::new();

    stack.push(10);
    stack.push(11);
    stack.push(12);

    println!("pop: {}", stack.pop());
    println!("pop: {}", stack.pop());
    println!("pop: {}", stack.pop());
}
