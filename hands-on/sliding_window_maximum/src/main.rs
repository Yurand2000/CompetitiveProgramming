use std::fs;
use std::time::Instant;

use sliding_window_maximum::{
    brute_force, brute_force_idiomatic, bst, gen_random_vector, heap, linear,
};

fn main() {
    let ns = [
        256,
        512,
        1024,
        2 * 1024,
        4 * 1024,
        8 * 1024,
        16 * 1024,
        32 * 1024,
    ];

    let mut ks: Vec<i32> = Vec::new();
    ks.push(1); ks.push(2);
    for i in (4..1025).step_by(4) { ks.push(i); }

    // Write csv header
    let mut output_text: String = "Method,n,k,elapsed\n".to_string();

    for &n in ns.iter() {
        println!("Now computing with n: {}", n);
        for &k in ks.iter() {
            if k as usize > n {
                continue;
            }
            let nums = gen_ordered_vector(n);

            // Brute force
            let (elapsed_times, _) = measure_elapsed_time(brute_force, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BruteForce", n, k, min_elapsed);
            output_text.push_str(&row);

            // Brute force idiomatic
            let (elapsed_times, _) = measure_elapsed_time(brute_force_idiomatic, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!(
                "{},{},{},{}\n",
                "BruteForceIdiomatic", n, k, min_elapsed
            );
            output_text.push_str(&row);

            // Bst
            let (elapsed_times, _) = measure_elapsed_time(bst, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BST", n, k, min_elapsed);
            output_text.push_str(&row);

            // Heap
            let (elapsed_times, _) = measure_elapsed_time(heap, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "Heap", n, k, min_elapsed);
            output_text.push_str(&row);

            // Linear
            let (elapsed_times, _) = measure_elapsed_time(linear, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "Linear", n, k, min_elapsed);
            output_text.push_str(&row);
        }
    }

    let output_path = "results.csv";
    fs::write(output_path, output_text).expect("Unable to write file");
}

const N_RUNS: usize = 5;

fn measure_elapsed_time(
    f: fn(vec: &Vec<i32>, k: i32) -> Vec<i32>,
    nums: &Vec<i32>,
    k: i32,
) -> (Vec<u128>, Vec<i32>) {
    let mut elapsed_times: Vec<u128> = Vec::with_capacity(N_RUNS);
    let mut results = Vec::new();
    for _ in 0..N_RUNS {
        let start = Instant::now();
        results = f(nums, k);
        let duration = start.elapsed().as_nanos();
        elapsed_times.push(duration);
    }

    (elapsed_times, results)
}

pub fn gen_ordered_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for i in (0..n).rev() {
        nums.push(i as i32);
    }

    nums
}