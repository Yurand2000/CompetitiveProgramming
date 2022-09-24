use std::cmp;
struct Solution;

struct Range
{
    left: i32,
    all: i32,
    right: i32,
    middle: i32
}

impl Range
{
    fn new(left: i32, all: i32, right: i32, middle: i32) -> Range
    {
        Range{left, all, right, middle}
    }

    fn max(&self) -> i32
    {
        cmp::max( cmp::max(self.left, self.all), cmp::max(self.right, self.middle) )
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32
    {
        let mut max = i32::MIN;
        let mut curr = 0;
        
        for num in nums.iter()
        {            
            curr += *num;
            if curr > max  {
                max = curr;
            }
            if curr < 0 {
                curr = 0;
            }
        }
        
        max
    }

    pub fn max_sub_array_divide_and_conquer(nums: Vec<i32>) -> i32
    {
        Solution::split_max_sub_array(nums.as_slice()).max()
    }
    
    fn split_max_sub_array(nums: &[i32]) -> Range
    {
        if nums.len() > 2
        {
            let (left, right) = nums.split_at( nums.len()/2 );
            let left = Solution::split_max_sub_array(left);
            let right = Solution::split_max_sub_array(right);
            Solution::merge_ranges(left, right)
        }
        else if nums.len() == 2
        {
            let left = nums[0];
            let right = nums[1];
            let sum = left + right;
            let positive_sum = cmp::max(sum, 0);

            if left < 0 && right < 0 {
                Range::new(positive_sum, sum, positive_sum, positive_sum)
            }
            else if left < 0 && right >= 0 {
                Range::new(positive_sum, sum, right, right)
            }
            else if left >= 0 && right < 0 {
                Range::new(left, sum, positive_sum, left)
            }
            else {
                Range::new(sum, sum, sum, sum)
            }
        }
        else
        {
            let val = nums[0];
            let positive_val = cmp::max(val, 0);
            Range::new(positive_val, val, positive_val, positive_val)
        }
    }
    
    fn merge_ranges(left: Range, right: Range) -> Range
    {
        let mut new_range = Range::new(left.left, left.all + right.all, right.right, left.right + right.left);

        let new_left = left.all + right.left;
        if new_left > left.left {
            new_range.left = new_left;
        }

        let new_right = left.right + right.all;
        if new_right > right.right {
            new_range.right = new_right;
        }

        new_range
    }
}

fn main() {
    let input: Vec<i32> = Vec::from([-2,1,-3,4,-1,2,1,-5,4]);
    let output = 6;

    assert_eq!(Solution::max_sub_array(input.clone()), output);
    assert_eq!(Solution::max_sub_array_divide_and_conquer(input), output);
    println!("Success!");
}
