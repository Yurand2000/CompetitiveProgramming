fn query(prefix_sum: &Vec<i32>, (l, r): (usize, usize)) -> i32 {
    prefix_sum[r-1] - prefix_sum[l-1]
}

fn compute_prefix_sum_vector(vec: Vec<bool>) -> Vec<i32>
{
    let mut sum = 0;
    vec.into_iter().map(|v| -> i32 {
        sum += if v { 1 } else { 0 };
        sum
    }).collect()
}

fn compute_next_equal_vector(vec: &str) -> Vec<bool>
{
    let mut last = '\0';
    vec.chars().into_iter().rev().map(|c| -> bool {
        let equal = c == last;
        last = c;
        equal
    }).rev().collect()
}

fn main() {
    let string = "......";
    let prefix_sum = compute_prefix_sum_vector(
        compute_next_equal_vector(&string)
    );

    assert_eq! (query(&prefix_sum, (3, 4)), 1 );
    assert_eq! (query(&prefix_sum, (2, 3)), 1 );
    assert_eq! (query(&prefix_sum, (1, 6)), 5 );
    assert_eq! (query(&prefix_sum, (2, 6)), 4 );
    
    let string = "#..###";
    let prefix_sum = compute_prefix_sum_vector(
        compute_next_equal_vector(&string)
    );

    assert_eq! (query(&prefix_sum, (1, 3)), 1 );
    assert_eq! (query(&prefix_sum, (5, 6)), 1 );
    assert_eq! (query(&prefix_sum, (1, 5)), 2 );
    assert_eq! (query(&prefix_sum, (3, 6)), 2 );
    assert_eq! (query(&prefix_sum, (3, 4)), 0 );

    println!("Success!");
}
