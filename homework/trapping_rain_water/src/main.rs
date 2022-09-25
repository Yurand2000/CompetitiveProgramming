use std::cmp;
use std::iter::Iterator;
use std::convert::TryInto;

struct Solution;
struct TrapInfo
{
    trap: i32,
    max_h: i32,
    inside: i32,
    count: i32
}

impl TrapInfo
{
    fn new(trap: i32, max_h: i32, inside: i32, count: i32) -> TrapInfo
    {
        TrapInfo{trap, max_h, inside, count}
    }
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {        
        let left_info = Solution::trap_one_way(height.iter());
        let right_info = Solution::trap_one_way(height.iter().rev());
        
        let middle_well_height = cmp::min(left_info.max_h, right_info.max_h);
        let middle_well_count: i32 = left_info.count - TryInto::<i32>::try_into(height.len()).unwrap() + right_info.count;
        let middle_well_inside = left_info.inside + right_info.inside - Solution::height_sum(height.iter());
        
        left_info.trap + right_info.trap + middle_well_height * middle_well_count - middle_well_inside
    }
    
    fn trap_one_way<'a, T>(iter: T) -> TrapInfo
    where T: Iterator<Item = &'a i32>
    {
        let mut left_bound = 0;
        let mut inside = 0;
        let mut inside_count = 0;
        
        let mut total_capacity = 0;
        for elem in iter {
            if *elem >= left_bound {
                let right_bound = *elem;
                let well_height = cmp::min(left_bound, right_bound);
                let well_capacity = inside_count * well_height - inside;
                total_capacity += well_capacity;
                
                left_bound = right_bound;
                inside = 0;
                inside_count = 0;
            }
            else {
                inside += *elem;
                inside_count += 1;
            }
        }
        
        TrapInfo::new(total_capacity, left_bound, inside, inside_count)
    }
    
    fn height_sum<'a, T>(iter: T) -> i32
    where T: Iterator<Item = &'a i32>
    {
        let mut height = 0;
        for elem in iter {
            height += *elem;
        }
        
        height
    }
}

fn main() {
    let input: Vec<i32> = Vec::from([0,1,0,2,1,0,1,3,2,1,2,1]);
    let output = 6;

    assert_eq!(Solution::trap(input), output);
    println!("Success!");
}
