/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::mem::swap;
use std::ops::Index;

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
        self.items.push(value);
        self.count += 1;
        let mut index = self.count;
        // self.comparator 是一个函数指针，其类型是 fn(&T, &T) -> bool。
        // 要调用这个函数指针，必须使用括号来明确表示这是一个函数调用
        // Rust 的语法设计中，函数指针和普通变量没有语法上的区别。
        // 因此，Rust 编译器需要一种方式来区分你是想调用一个函数，还是仅仅访问一个变量。
        loop {
            let parent = self.parent_idx(index);
            if parent == 0 { break; }
            if (self.comparator)(self.items.index(index), self.items.index(parent)) {
                // swap(&mut self.items[index], &mut self.items[self.parent_idx(index)]);
                self.items.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        // 判断当前节点下有没有子节点
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 当有子节点时才能用这个函数
        if !self.children_present(idx) {
            return 0;
        }

        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果右子节点存在且右子节点的值小于左子节点的值
        if right <= self.count && (self.comparator)(self.items.index(right) , self.items.index(left)) {
            right
        } else {
            left
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.items.swap_remove(1);
        self.count -= 1;
        let mut index = 1usize; // 下一个要交换的索引
        let mut temp = 1usize;
        while self.children_present(index) {
            temp = index;
            index = self.smallest_child_idx(index);
            if (self.comparator)(self.items.index(index), self.items.index(temp)) {
                self.items.swap(index, temp);
            } else {
                break;
            }
        }
        // self.count -= 1;
        Some(result)
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
