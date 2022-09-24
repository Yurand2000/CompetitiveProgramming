/* PROBLEM:
 * Given an array A of positive integers. Your task is to find the leaders in the array.
 * An element of array is leader if it is greater than or equal to all the elements to its right side.
 * The rightmost element is always a leader. 
 */

use std::fs::File;
use std::io::prelude::*;
use std::str::SplitWhitespace;

fn solution(input: SplitWhitespace<'_>) -> Vec<i32>
{
    let mut leaders: Vec<i32> = Vec::new();

    let mut max = -1;
    for element in input.rev()
    {
        let curr: i32 = element.parse().unwrap();
        if curr >= max
        {
            max = curr;
            leaders.push(curr);
        }
    }
    leaders.reverse();

    leaders
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    {
        let mut input_file = File::open("resources/leaders_in_array_input.txt").unwrap();
        input_file.read_to_string(&mut input).unwrap();
    }
    {
        let mut output_file = File::open("resources/leaders_in_array_output.txt").unwrap();
        output_file.read_to_string(&mut output).unwrap();
    }

    let mut it = input.split_whitespace();
    let _array_size: usize = it.next().unwrap().parse().unwrap();

    let leaders = solution(it);

    let mut it = output.split_whitespace();
    let out_array_size: usize = it.next().unwrap().parse().unwrap();

    assert_eq!(leaders.len(), out_array_size, "Leaders array has different size then required output");

    for i in 0..out_array_size {
        let curr_leader = leaders[i];
        let curr_output: i32 = it.next().unwrap().parse().unwrap();

        assert_eq!(curr_leader, curr_output, "Leader different at position {}! Expected: {}; Got: {}", i, curr_output, curr_leader);
    }

    println!("Success!");
}
