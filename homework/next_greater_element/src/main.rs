struct Solution;

impl Solution {
    pub fn trivial_next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut max: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            max.push( Solution::trivial_find_next_greater_element(&nums, i) );
        }
        max
    }

    fn trivial_find_next_greater_element(nums: &Vec<i32>, n: usize) -> i32
    {
        for i in 1..nums.len() {
            let i = (i + n) % nums.len();
            let next = nums[i];

            if next > nums[n] {
                return next;
            }
        }

        -1
    }
}

fn main() {
    let input = vec![1,2,3,4,3];
    let output = vec![2,3,4,-1,4];

    assert_eq!(output, Solution::trivial_next_greater_elements(input.clone()));

    let input = vec![5,4,3,2,1];
    let output = vec![-1,5,5,5,5];
    assert_eq!(output, Solution::trivial_next_greater_elements(input.clone()));

    let input = vec![2,2,2,2,1];
    let output = vec![-1,-1,-1,-1,2];
    assert_eq!(output, Solution::trivial_next_greater_elements(input.clone()));

    println!("Success!");
}

/*impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32>
    {
        if Solution::any_different(&nums)
        {
            let mut maxs: Vec<Option<i32>> = vec![None; nums.len()];

            let mut curr: usize = 0;
            while curr < nums.len()
            {
                curr = Solution::find_next_greater_elements(&nums, curr, &mut maxs);
            }
            maxs.iter().map(|o| -> i32 { o.unwrap() }).collect()
        }
        else
        {
            vec![-1; nums.len()]
        }
    }

    fn find_next_greater_elements(nums: &Vec<i32>, curr: usize, maxs: &mut Vec<Option<i32>>) -> usize
    {
        let mut next = (curr + 1) % nums.len();
        while maxs[curr].is_none() && nums[curr] >= nums[next]
        {
            next = Solution::find_next_greater_elements(nums, next, maxs);

            if next == curr
            {
                maxs[curr] = Some(-1);
                return curr + 1;
            }
        }

        if maxs[curr].is_none() {
            maxs[curr] = Some(nums[next]);
            next
        }
        else {
            curr + 1
        }
    }

    fn any_different(nums: &Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let n = nums[0];
        for i in 1..nums.len() {
            if nums[i] != n {
                return true;
            }
        }
        false
    }
}*/