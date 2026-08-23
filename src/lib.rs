use std::cmp::PartialOrd;

pub struct Heap<T> {
    pub heap: Vec<T>,
}

impl<T> Heap<T>
where
    T: PartialOrd + Copy,
{
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn push(&mut self, value: T)
    where
        T: PartialOrd,
    {
        self.heap.push(value);
        let mut i = self.heap.len() - 1;
        while i > 0 && self.heap[i] < self.heap[(i - 1) / 2] {
            let parent = (i - 1) / 2;
            self.swap(i, parent);
            i = parent;
        }
    }

    pub fn remove_min(&mut self) -> T {
        let max = self.heap[0];
        let len_heap = self.heap.len() - 1;
        self.heap.swap(0, len_heap);
        self.heap.pop();
        let mut i = 0;
        while 2 * i + 1 < self.heap.len() {
            let mut j = 2 * i + 1;
            if j + 1 < self.heap.len() && self.heap[j] < self.heap[j + 1] {
                j += 1;
            }
            if self.heap[i] <= self.heap[j] {
                break;
            }
            self.swap(i, j);
            i = j;
        }
        max
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.remove_min(), 1);
        assert_eq!(heap.remove_min(), 2);
        assert_eq!(heap.remove_min(), 3);
    }
}
