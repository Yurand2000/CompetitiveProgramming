use crate::*;
use std::io::{self, BufReader, BufRead};

#[inline(always)]
pub fn read_input_from_stdin() -> (Vec<i32>, Vec<Operation>, Vec<Query>) {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines().map(|next| next.unwrap() );
    read_input(&mut lines)
}

pub fn read_input<T>(input: &mut T) -> (Vec<i32>, Vec<Operation>, Vec<Query>)
    where T: Iterator<Item = String>
{
    let (size, ops, queries) = read_size_from_io(input);
    let array = read_array_from_io(input, size);
    let ops = read_ops_from_io(input, ops);
    let queries = read_queries_from_io(input, queries);

    (array, ops, queries)
}

fn read_size_from_io<T>(input: &mut T) -> (usize, usize, usize)
    where T: Iterator<Item = String>
{
    let sizes_str = input.next().unwrap();

    let mut sizes_str = sizes_str.split_whitespace();
    let size = sizes_str.next().unwrap().parse().unwrap();
    let ops = sizes_str.next().unwrap().parse().unwrap();
    let queries = sizes_str.next().unwrap().parse().unwrap();

    (size, ops, queries)
}

fn read_array_from_io<T>(input: &mut T, size: usize) -> Vec<i32>
    where T: Iterator<Item = String>
{
    let array_str = input.next().unwrap();

    let mut array_str = array_str.split_whitespace();
    let mut array = Vec::with_capacity(size);
    for _ in 0..size {
        array.push(array_str.next().unwrap().parse().unwrap());
    }

    array
}

fn read_ops_from_io<T>(input: &mut T, ops: usize) -> Vec<Operation>
    where T: Iterator<Item = String>
{
    let mut ops_vec = Vec::with_capacity(ops);

    for _ in 0..ops
    {
        let ops_str = input.next().unwrap();
        let mut ops_reader = ops_str.split_whitespace();

        ops_vec.push(Operation{
            left: ops_reader.next().unwrap().parse().unwrap(),
            right: ops_reader.next().unwrap().parse().unwrap(),
            value: ops_reader.next().unwrap().parse().unwrap()
        });
    }

    ops_vec
}

fn read_queries_from_io<T>(input: &mut T, queries: usize) -> Vec<Query>
    where T: Iterator<Item = String>
{
    let mut queries_vec = Vec::with_capacity(queries);

    for _ in 0..queries
    {
        let query_str = input.next().unwrap();
        let mut query_reader = query_str.split_whitespace();

        queries_vec.push(Query{
            left: query_reader.next().unwrap().parse().unwrap(),
            right: query_reader.next().unwrap().parse().unwrap()
        });
    }

    queries_vec
}