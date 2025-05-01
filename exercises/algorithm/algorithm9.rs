/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // Add item to the end of the heap
        self.items.push(value);
        self.count += 1;
        
        // Heapify up
        let mut idx = self.count;
        while idx > 1 {
            let parent = self.parent_idx(idx);
            
            // If parent should be above this item, we're done
            if (self.comparator)(&self.items[parent], &self.items[idx]) {
                break;
            }
            
            // Otherwise, swap and continue
            self.items.swap(parent, idx);
            idx = parent;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // If no children, return the same index
        if !self.children_present(idx) {
            return idx;
        }
        
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        // If right child doesn't exist, return left
        if right > self.count {
            return left;
        }
        
        // Return the smaller (or larger for max heap) of the two children
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        // Take the top item
        let last_item = self.items[self.count].clone();
        let item = std::mem::replace(&mut self.items[1], last_item);
        self.items.pop();
        self.count -= 1;
        
        // If heap is now empty, we're done
        if self.is_empty() {
            return Some(item);
        }
        
        // Heapify down
        let mut idx = 1;
        while self.children_present(idx) {
            let child = self.smallest_child_idx(idx);
            
            // If this item should be above the child, we're done
            if (self.comparator)(&self.items[idx], &self.items[child]) {
                break;
            }
            
            // Otherwise, swap and continue
            self.items.swap(idx, child);
            idx = child;
        }
        
        Some(item)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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