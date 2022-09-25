use std::{convert::TryInto, collections::VecDeque};

struct Solution;

struct SlidingWindowQueue
{
    deque: VecDeque<(i32, i32)>
}

impl SlidingWindowQueue
{
    fn new() -> SlidingWindowQueue
    {
        SlidingWindowQueue{deque: VecDeque::new()}
    }

    fn push(&mut self, val: i32, pos: i32)
    {
        if self.deque.len() > 0 && self.deque.back().unwrap().0 < val {
            self.deque.pop_back();
            self.push(val, pos)
        }
        else {
            self.deque.push_back( (val, pos) );
        }
    }

    fn pop(&mut self, left_pos: i32)
    {
        if self.deque.len() > 0 && self.deque.front().unwrap().1 <= left_pos {
            self.deque.pop_front();
            self.pop(left_pos);
        }
    }

    fn max(&self) -> i32
    {
        self.deque.front().unwrap().0
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut maximums: Vec<i32> = Vec::new();

        let mut queue = SlidingWindowQueue::new();
        for i in 0..nums.len()
        {
            let (curr, pos): (_, i32) = (nums[i], i.try_into().unwrap());

            queue.pop(pos - k);
            queue.push(curr, pos);

            if pos >= k - 1 {
                maximums.push(queue.max());
            }
        }

        println!("{:?}", maximums);

        maximums
    }
}

fn main() {
    let input: Vec<i32> = Vec::from([1,3,-1,-3,5,3,6,7]);
    let k = 3;
    let output: Vec<i32> = Vec::from([3,3,5,5,6,7]);

    assert_eq!(Solution::max_sliding_window(input, k), output);
    println!("Success!");
}
