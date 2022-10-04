struct Solution;

impl Solution {
    pub fn towers(mut bricks: Vec<i32>) -> (i32, i32)
    {
        bricks.sort();

        let mut max_h = 0;
        let mut curr_h = 0;
        let mut count = 0;
        let mut prec = -1;
        for brick in bricks
        {
            if brick != prec {
                if max_h < curr_h {
                    max_h = curr_h;
                }
                prec = brick;
                count += 1;
                curr_h = 1;
            }
            else {
                curr_h += 1;
            }
        }

        if max_h < curr_h {
            max_h = curr_h;
        }

        (max_h, count)
    }
}

fn main() {
    let input = vec![1, 2, 3];
    let output = (1, 3);

    assert_eq!(Solution::towers(input), output);

    let input = vec![6, 5, 6, 7];
    let output = (2, 3);

    assert_eq!(Solution::towers(input), output);

    println!("Success");
}
