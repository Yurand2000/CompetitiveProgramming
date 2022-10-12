use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut window: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity( k );
    let mut maximums: Vec<i32> = Vec::with_capacity( number_of_windows(nums.len(), k) );

    for (pos, &val) in nums[..k].iter().enumerate() {
        window.push( (val, pos) );
    }

    maximums.push( window.peek().unwrap().0 );
    for (pos, &val) in nums[k..].iter().enumerate() {
        window.push( (val, pos + k) );

        while window.peek().unwrap().1 <= pos {
            window.pop();
        }

        maximums.push( window.peek().unwrap().0 );
    }

    maximums
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
    let mut maximums: Vec<i32> = Vec::with_capacity( number_of_windows(nums.len(), k) );

    for &i in nums[..k].iter() {
        bst.insert(i);
    }

    maximums.push( *bst.max().unwrap() );
    for i in k..nums.len() {
        bst.remove(&nums[i - k]);
        bst.insert(nums[i]);
        maximums.push( *bst.max().unwrap() );
    }

    maximums
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut maximums: Vec<i32> = Vec::with_capacity( number_of_windows(nums.len(), k) );
    let mut queue = SlidingWindowQueue::with_capacity(k);

    for (pos, &curr) in nums[..k].iter().enumerate() {
        queue.push(curr, pos);
    }

    maximums.push( queue.max() );
    for (pos, &curr) in nums[k..].iter().enumerate() {
        queue.pop(pos);
        queue.push(curr, pos + k);
        maximums.push( queue.max() );
    }

    maximums
}

fn number_of_windows(nums: usize, k: usize) -> usize {
    nums - k + 1
}

struct SlidingWindowQueue {
    deque: VecDeque<(i32, usize)>,
}

impl SlidingWindowQueue {
    fn with_capacity(capacity: usize) -> SlidingWindowQueue {
        SlidingWindowQueue {
            deque: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, val: i32, pos: usize) {
        match self.deque.back() {
            Some(&(n, _)) if n < val => {
                self.deque.pop_back();
                self.push(val, pos);
            }
            _ => self.deque.push_back( (val, pos) )
        }
    }

    fn pop(&mut self, left_pos: usize) {
        match self.deque.front() {
            Some(&(_, pos)) if pos <= left_pos => {
                self.deque.pop_front();
                self.pop(left_pos);
            }
            _ => (),
        }
    }

    fn max(&self) -> i32 {
        self.deque.front().unwrap().0
    }
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _test_function(n: usize, k: i32, funct: fn(&Vec<i32>, i32) -> Vec<i32>) {
        let v = gen_random_vector(n);

        let results = funct(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    fn test_function(funct: fn(&Vec<i32>, i32) -> Vec<i32>) {
        _test_function(100, 1, funct);
        _test_function(100, 3, funct);
        _test_function(150, 30, funct);
        _test_function(200, 70, funct);
        _test_function(200, 100, funct);
        _test_function(200, 150, funct);
        _test_function(200, 200, funct);
    }

    #[test]
    fn test_idiomatic_version() {
        test_function(brute_force_idiomatic);
    }

    #[test]
    fn test_heap_version() {
        test_function(heap);
    }

    #[test]
    fn test_bst_version() {
        test_function(bst);
    }

    #[test]
    fn test_linear_version() {
        test_function(linear);
    }
}
