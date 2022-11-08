fn compute_sum(vec: &Vec<i32>, priorities: &Vec<i32>) -> i32 {
    vec.iter().zip(priorities.iter()).map(|(&val, &times)| val * times).sum()
}

fn permute_by_priority(vec: &Vec<i32>, priorities: &Vec<i32>) -> Vec<i32>
{
    //sort values
    let mut vec = vec.clone();
    vec.sort();

    //sort priorities vector (with their position, so defining a permutation)
    let mut priorities = priorities.iter().enumerate()
        .map(|(a,&b)| (b,a)).collect::<Vec<(i32, usize)>>();
    priorities.sort();

    //permute the values
    let mut permutation: Vec<i32> = vec![0; vec.len()];
    for (val, (_, pos)) in vec.into_iter().zip(priorities.into_iter())
    {
        permutation[pos] = val;
    }

    permutation
}

fn build_priority_prefix_sum_inverse(len: usize, queries: Vec<(usize, usize)>) -> Vec<i32>
{
    let mut vec = vec![0; len];
    for (start, end) in queries.into_iter() {
        vec.get_mut(start).map(|v| *v += 1 );
        vec.get_mut(end + 1).map(|v| *v -= 1 );
    }
    vec
}

fn build_priority_vec(prefix_vec: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    prefix_vec.into_iter().map(|v| { sum += v; sum } ).collect()
}

fn main() {
    let a = vec![5, 3, 2];
    let queries = [(1, 2), (2, 3), (1, 3)]
        .iter().map(|(a,b)| (a-1, b-1)).collect();
    let priority_vec = build_priority_vec(
        build_priority_prefix_sum_inverse(a.len(), queries)
    );

    let a = permute_by_priority(&a, &priority_vec);
    assert_eq!(compute_sum(&a, &priority_vec), 25);

    let a = vec![5, 2, 4, 1, 3];
    let queries = [(1, 5), (2, 3), (2, 3)]
        .iter().map(|(a,b)| (a-1, b-1)).collect();
    let priority_vec = build_priority_vec(
        build_priority_prefix_sum_inverse(a.len(), queries)
    );

    let a = permute_by_priority(&a, &priority_vec);
    assert_eq!(compute_sum(&a, &priority_vec), 33);

    println!("Success!");
}
