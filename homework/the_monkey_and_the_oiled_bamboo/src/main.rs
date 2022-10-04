use std::cmp::Ordering;

struct Solution;

impl Solution
{
    pub fn min_force(rung_heigt_diffs: &Vec<i32>) -> i32 {
        let mut force = 0;
        for elem in rung_heigt_diffs.iter().rev()
        {
            if force == *elem
            {
                force += 1;
            }
            else if force < *elem
            {
                force = *elem;
            }
        }
        force
    }

    pub fn bin_search_min_force(rung_heigt_diffs: &Vec<i32>) -> i32 {
        if rung_heigt_diffs.is_empty() {
            0
        } else {
            Solution::rec_bin_search_min_force(&rung_heigt_diffs, 0, *rung_heigt_diffs.iter().max().unwrap() + 1)
        }
    }

    fn bin_search_check(rung_heigt_diffs: &Vec<i32>, mut k: i32) -> bool {
        for elem in rung_heigt_diffs {
            match k.cmp(elem) {
                Ordering::Greater => {},
                Ordering::Equal => { k -= 1; },
                Ordering::Less => { return false; },
            }
        }
        true
    }

    fn rec_bin_search_min_force(rung_heigt_diffs: &Vec<i32>, min: i32, max: i32) -> i32 {
        if min == max {
            min
        }
        else {
            let middle = (min + max) / 2;
            if Solution::bin_search_check(rung_heigt_diffs, middle) {
                Solution::rec_bin_search_min_force(rung_heigt_diffs, min, middle)
            }
            else {
                Solution::rec_bin_search_min_force(rung_heigt_diffs, middle + 1, max)    
            }
        }
    }
}

fn main() {
    let input = vec![1, 6, 7, 11, 13];
    let input = diffs_from_input(input);
    let output = 5;

    assert_eq!(Solution::bin_search_min_force(&input), output);
    assert_eq!(Solution::min_force(&input), output);

    let input = vec![3, 9, 10, 14];
    let input = diffs_from_input(input);
    let output = 6;

    assert_eq!(Solution::bin_search_min_force(&input), output);
    assert_eq!(Solution::min_force(&input), output);

    let input = vec![5, 4, 3, 2];
    let output = 5;

    assert_eq!(Solution::bin_search_min_force(&input), output);
    assert_eq!(Solution::min_force(&input), output);

    let input = vec![5, 4, 4, 2];
    let output = 6;

    assert_eq!(Solution::bin_search_min_force(&input), output);
    assert_eq!(Solution::min_force(&input), output);

    println!("Success");
}

fn diffs_from_input(input: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut prec = 0;
    for elem in input {
        vec.push(elem - prec);
        prec = elem;
    }
    vec
}
