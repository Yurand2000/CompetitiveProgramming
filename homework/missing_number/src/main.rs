use std::convert::TryInto;

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32
    {
        let n: i32 = nums.len().try_into().unwrap();
        let expected: i32 = n * (n+1) / 2;
        
        let mut sum = 0;
        for num in nums.iter()
        {
            sum += num;
        }
        
        expected - sum
    }
}

fn main() {
    let input: Vec<i32> = Vec::from([9,6,4,2,3,5,7,0,1]);
    let output = 8;

    assert_eq!(Solution::missing_number(input), output);
    println!("Success!");
}
