//! Vector based Heap implementation.

use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T: Debug> {
    data: Vec<T>,
}

impl<T: Ord + Clone + Debug> Heap<T> {
    /// Constructor for creating [Heap] for specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let data = Vec::with_capacity(capacity);
        Heap { data }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let result = self.data.swap_remove(0);

        self.pull_down(0); // repair heap after remove

        Some(result)
    }

    /// Pushes an item onto the binary heap.
    pub fn push(&mut self, item: T) {
        let item_idx = self.push_back(item);
        self.lift_up(item_idx); // repair heap after push
    }

    /// Returns the length of the binary heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    //
    // Private methods
    //

    /// Takes an item at `idx` and either moves it up the heap if item less than
    /// it's parent or does nothing otherwise.
    fn lift_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let parent_idx: usize = (idx - 1) / 2;

        if &self.data[parent_idx] < &self.data[idx] {
            return;
        }

        self.data.swap(idx, parent_idx);

        self.lift_up(parent_idx);
    }

    /// Takes an item at `idx` and either moves it down the heap if item grater
    /// than it's parent or does nothing otherwise.
    fn pull_down(&mut self, parent_idx: usize) {
        let left_child_idx = ((parent_idx + 1) * 2) - 1;
        let left_child_exists = left_child_idx < self.len();

        let right_child_idx = (parent_idx + 1) * 2;
        let right_child_exists = right_child_idx < self.len();

        if left_child_exists && &self.data[left_child_idx] < &self.data[parent_idx] {
            let min_child_idx =
                if right_child_exists && &self.data[right_child_idx] < &self.data[left_child_idx] {
                    right_child_idx
                } else {
                    left_child_idx
                };

            self.data.swap(parent_idx, min_child_idx);
            self.pull_down(min_child_idx);
        } else if right_child_exists && &self.data[right_child_idx] < &self.data[parent_idx] {
            self.data.swap(parent_idx, right_child_idx);
            self.pull_down(right_child_idx);
        }
    }

    /// Push item to last position and increment `last_item_idx`. Returns `idx`
    /// of new pushed item.
    fn push_back(&mut self, item: T) -> usize {
        self.data.push(item);
        self.data.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use algorithms::heap::vector_based_heap::Heap;

    #[test]
    fn heap_test() {
        let src_data: Vec<String> = (10..99)
            .rev()
            .collect::<Vec<usize>>()
            .iter()
            .map(|el| el.to_string())
            .collect();

        let mut heap: Heap<String> = Heap::with_capacity(90);
        assert_eq!(heap.len(), 0);

        // push data to Heap
        for item in &src_data {
            heap.push(item.clone())
        }
        assert_eq!(heap.len(), src_data.len());

        // pop data from Heap
        let mut sorted_vector: Vec<String> = Vec::with_capacity(90);
        loop {
            if let Some(item) = heap.pop() {
                sorted_vector.push(item)
            } else {
                break;
            }
        }

        // do sort vector for comparing it with heap-sorted vector
        let mut expected_vec = sorted_vector.clone();
        expected_vec.sort();
        assert_eq!(sorted_vector, expected_vec)
    }

}
