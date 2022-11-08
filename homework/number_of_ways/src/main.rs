fn compute_prefix_sum(vec: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    vec.into_iter().map(|v| -> i32 {
        sum += v;
        sum
    }).collect()
}

fn number_of_ways(mut sum_vec: Vec<i32>) -> i32 {
    if sum_vec.is_empty() || sum_vec.last().unwrap() % 3 != 0 {
        0
    } else {
        let sum_third = sum_vec.last().unwrap() / 3;
        if sum_third == 0 {
            let len = sum_vec.len() as i32;
            sum_vec.iter_mut().enumerate()
                .map(|(i, v)| { *v += len * i as i32; })
            .count();
        }

        let mut count = 0;
        let mut suffixes = sum_vec.iter().rev().map(|&v| -> i32 {
            if v == sum_third * 2 { count += 1; }
            count
        }).collect::<Vec<i32>>();
        suffixes.reverse();

        sum_vec.iter().enumerate().filter(|&(_, &v)| -> bool { v == sum_third })
        .map(|(i, _)| -> i32 {
            suffixes[i]
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
