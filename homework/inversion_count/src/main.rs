struct Solution;

impl Solution
{
    pub fn inversion_count(vec: &Vec<i32>) -> i32
    {
        let mut vec = vec.clone();
        Solution::rec_inversion_count(&mut vec)
    }

    pub fn rec_inversion_count(vec: &mut [i32]) -> i32
    {
        if vec.len() <= 1 {
            0
        }
        else {
            let middle = vec.len() / 2;
            let (left, right) = vec.split_at_mut(middle);

            let left = Solution::rec_inversion_count(left);
            let right = Solution::rec_inversion_count(right);

            Solution::inversion_count_merge(vec, middle) + left + right
        }
    }

    pub fn inversion_count_merge(vec: &mut [i32], mid: usize) -> i32
    {
        let (left, right) = vec.split_at(mid);
        let (left, right) = (left.to_vec(), right.to_vec());

        let mut inversions: usize = 0;

        let mut v_ptr: usize = 0;
        let mut l_ptr: usize = 0;
        let mut r_ptr: usize = 0;
        while l_ptr < left.len() && r_ptr < right.len()
        {
            if left[l_ptr] < right[r_ptr]
            {
                vec[v_ptr] = left[l_ptr];
                l_ptr += 1;
            }
            else
            {
                vec[v_ptr] = right[r_ptr];
                r_ptr += 1;

                inversions += left.len() - l_ptr;
            }
            v_ptr += 1;
        }

        for i in l_ptr..left.len() {
            vec[v_ptr] = left[i];
            v_ptr += 1;
        }

        for i in r_ptr..right.len() {
            vec[v_ptr] = right[i];
            v_ptr += 1;
        }

        inversions as i32
    }
}

fn main() {
    let input = vec![8, 4, 2, 1];
    let output = 6;

    assert_eq!(Solution::inversion_count(&input), output);

    let input = vec![1, 20, 6, 4, 5];
    let output = 5;

    assert_eq!(Solution::inversion_count(&input), output);

    println!("Success");
}
