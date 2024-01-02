use std::cmp::Ordering;
use std::thread::panicking;
macro_rules! left_child {
    ($index: ident) => {
        $index << 1
    };
}

macro_rules! right_child {
    ($index: ident) => {
        ($index << 1) + 1
    };
}

macro_rules! parent {
    ($index: ident) => {
        $index >> 1
    };
}
struct BinaryHeap<T: Ord + Default> {
    size: usize,
    data: Vec<T>,
}

impl<T: Ord + Default> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap {
            size: 0,
            data: vec![T::default()],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn move_up(&mut self, mut index: usize) {
        loop {
            let parent_index = parent!(index);
            if parent_index == 0 {
                break;
            }
            if let Ordering::Less = self.data[index].cmp(&self.data[parent_index]) {
                self.data.swap(parent_index, index);
            }
            index = parent_index;
        }
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        self.data.push(val);
        self.move_up(self.size);
    }

    fn move_down(&mut self, mut index: usize) {
        loop {
            let left_index = left_child!(index);
            if left_index > self.size {
                break;
            }
            let right_index = right_child!(index);
            let min_index = match (left_index, right_index) {
                (left_index, right_index) if right_index > self.size => left_index,
                (left_index, right_index) => {
                    if let Ordering::Less = self.data[left_index].cmp(&self.data[right_index]) {
                        left_index
                    } else {
                        right_index
                    }
                }
            };
            if let Ordering::Greater = self.data[index].cmp(&self.data[min_index]) {
                self.data.swap(index, min_index);
            }
            index = min_index;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            1 => {
                self.size -= 1;
                self.data.pop()
            }
            size => {
                self.size -= 1;
                self.data.swap(1, size);
                let data = self.data.pop();
                self.move_down(1);
                data
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_heap::BinaryHeap;

    #[test]
    fn test_marco() {
        let a = 1;
        assert_eq!(left_child!(a), 2);
        assert_eq!(right_child!(a), 3);
    }

    #[test]
    fn test_push_and_pop() {
        let mut heap = BinaryHeap::new();
        heap.push(2);
        heap.push(8);
        heap.push(1);
        heap.push(9);
        heap.push(5);
        heap.push(4);
        heap.push(6);
        heap.push(3);
        while !heap.is_empty() {
            println!("{:?}", heap.pop());
        }
    }
}
