fn compute_prefix_sum(vec: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    vec.into_iter().map(|v| -> i32 {
        sum += v;
        sum
    }).collect()
}

fn number_of_ways(sum_vec: Vec<i32>) -> i32 {
    if sum_vec.is_empty() || sum_vec.last().unwrap() % 3 != 0 {
        0
    } else {
        let sum_third = sum_vec.last().unwrap() / 3;
        sum_vec.iter().enumerate().filter(|&(_, &v)| -> bool { v == sum_third })
        .map(|(i, _)| -> i32 {
            sum_vec.iter().skip(i+1).rev().skip(1)
                .filter(|&&v| -> bool {
                    v == sum_third * 2
                }).count() as i32
        }).sum()
    }
}

fn main() {
    let a = compute_prefix_sum(vec![1, 2, 3, 0, 3]);
    assert_eq!(number_of_ways(a), 2);

    let a = compute_prefix_sum(vec![0, 1, -1, 0]);
    assert_eq!(number_of_ways(a), 1);

    let a = compute_prefix_sum(vec![4, 1]);
    assert_eq!(number_of_ways(a), 0);

    println!("Success!");
}
