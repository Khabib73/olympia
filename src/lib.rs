use std::cmp::PartialOrd;

pub struct Heap<T> {
    pub heap: Vec<T>,
}

impl<T> Heap<T> where T: PartialOrd + Copy {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn push(&mut self, value: T)
    where
        T: PartialOrd,
    {
        self.heap.push(value);
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i] > self.heap[parent] {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
       self.heap.swap(i, j);
    }
}