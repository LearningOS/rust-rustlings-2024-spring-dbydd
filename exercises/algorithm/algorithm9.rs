/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

pub struct Heap<T>
where
    T: Default + Debug,
{
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove(&mut self) -> Option<T> {
        if (self.len() == 0) {
            return None;
        }
        let mut tail = self.items.len() - 1;
        let mut current = 1;
        let mut_ptr = self.items.as_mut_ptr();
        unsafe {
            std::ptr::swap(mut_ptr.offset(1), mut_ptr.offset(tail as isize));
            let pop = self.items.pop();
            while self.children_present(current) {
                let lc = self.left_child_idx(current);
                let rc = self.right_child_idx(current);
                match (
                    (self.comparator)(&self.items[current], &self.items[lc]),
                    (self.comparator)(&self.items[current], &self.items[rc]),
                ) {
                    (true, true) => break,
                    (true, false) => {
                        Self::swap(mut_ptr, rc, current);
                        current = rc;
                        println!("downfilled:{:?}", self.items);
                    }
                    (false, true) => {
                        Self::swap(mut_ptr, lc, current);
                        current = lc;
                        println!("downfilled:{:?}", self.items);
                    }
                    (false, false) => {
                        if (self.comparator)(&self.items[lc], &self.items[rc]) {
                            Self::swap(mut_ptr, lc, current);
                            current = lc;
                            println!("downfilled:{:?}", self.items);
                        } else {
                            Self::swap(mut_ptr, rc, current);
                            current = rc;
                            println!("downfilled:{:?}", self.items);
                        }
                    }
                };
            }
            println!("popped:{:?},vec:{:?}", pop, self.items);
            pop
        }
    }

    fn swap(myself: *mut T, n1: usize, n2: usize) {
        unsafe { std::ptr::swap(myself.offset(n1 as isize), myself.offset(n2 as isize)) }
    }

    pub fn add(&mut self, value: T) {
        println!("insert!:{:?}", value);
        if (self.items.is_empty()) {
            self.items.push(value);
            return;
        }
        self.items.push(value);
        let mut idx = self.items.len() - 1;
        let mut parent_idx = self.parent_idx(idx);
        while idx >= parent_idx {
            println!("upfilled:{:?}", self.items);
            let mut parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                let mut_ptr = self.items.as_mut_ptr();
                unsafe {
                    std::ptr::swap(
                        mut_ptr.offset(idx as isize),
                        mut_ptr.offset(parent_idx as isize),
                    );
                }
                idx = parent_idx;
            } else {
                return;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        if (idx == 1) {
            return 1;
        }
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.len()
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        let ret = self.left_child_idx(idx) + 1;
        if (ret <= self.len()) {
            ret
        } else {
            ret - 1
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.remove()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
