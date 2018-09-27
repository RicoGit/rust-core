
pub struct SimpleStack {
    stack: [i32; 10],
    head_ptr: usize
}

impl SimpleStack {
    pub fn new() -> Self {
        let empty_arr: [i32; 10] = [0; 10];
        SimpleStack { stack: empty_arr, head_ptr: 0 }
    }

    pub fn push(&mut self, elem: i32) -> () {
        self.head_ptr = self.head_ptr + 1;
        self.stack[self.head_ptr] = elem;
    }

    pub fn pop(&mut self) -> i32 {
        let next = self.stack[self.head_ptr];
        self.head_ptr = self.head_ptr - 1;
        return next
    }
}


#[test]
fn complex_test() {

    let mut stack = SimpleStack::new();

    stack.push(10);
    stack.push(11);
    stack.push(12);

    assert_eq!(stack.pop(), 12);
    assert_eq!(stack.pop(), 11);
    assert_eq!(stack.pop(), 10);

}