#![allow(dead_code)]

pub mod vector_based_heap;

use algorithms::heap::vector_based_heap::Heap;

pub fn demo_vector_based_heap() {
    let mut heap = Heap::with_capacity(10);

    heap.push("Z");
    heap.push("Y");
    heap.push("X");
    heap.push("C");
    heap.push("B");
    heap.push("A");

    let mut result: Vec<String> = Vec::with_capacity(90);

    loop {
        if let Some(item) = heap.pop() {
            result.push(item.to_string());
        } else {
            break;
        };
    }

    println!("heap-sorted vector: {:?}", result);
}
