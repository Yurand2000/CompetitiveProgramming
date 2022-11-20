use crate::Query;
use std::io::{self, BufReader, BufRead};

#[inline(always)]
pub fn read_input_from_stdin() -> (Vec<i32>, Vec<Query>) {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines().map(|next| next.unwrap() );
    read_input(&mut lines)
}

pub fn read_input<'a, T>(input: &mut T) -> (Vec<i32>, Vec<Query>)
    where T: Iterator<Item = String>
{
    let (size, queries) = read_size_from_io(input);
    let array = read_array_from_io(input, size);
    let queries = read_queries_from_io(input, queries);

    (array, queries)
}

fn read_size_from_io<'a, T>(input: &mut T) -> (usize, usize)
    where T: Iterator<Item = String>
{
    let sizes_str = input.next().unwrap();

    let mut sizes_str = sizes_str.trim().split_ascii_whitespace();
    let size = sizes_str.next().unwrap().parse().unwrap();
    let queries = sizes_str.next().unwrap().parse().unwrap();

    (size, queries)
}

fn read_array_from_io<'a, T>(input: &mut T, size: usize) -> Vec<i32>
    where T: Iterator<Item = String>
{
    let array_str = input.next().unwrap();

    let mut array_str = array_str.trim().split_ascii_whitespace();
    let mut array = Vec::with_capacity(size);
    for _ in 0..size {
        array.push(array_str.next().unwrap().parse().unwrap());
    }

    array
}

fn read_queries_from_io<'a, T>(input: &mut T, queries: usize) -> Vec<Query>
    where T: Iterator<Item = String>
{
    let mut queries_vec = Vec::with_capacity(queries);

    for _ in 0..queries
    {
        let query_str = input.next().unwrap();
        let mut query_reader = query_str.trim().split_ascii_whitespace();

        match query_reader.next().unwrap().parse().unwrap() {
            0 => {
                queries_vec.push( Query::Update(
                    query_reader.next().unwrap().parse().unwrap(),
                    query_reader.next().unwrap().parse().unwrap(),
                    query_reader.next().unwrap().parse().unwrap()
                ) )
            },
            1 => { 
                queries_vec.push( Query::Max(
                    query_reader.next().unwrap().parse().unwrap(),
                    query_reader.next().unwrap().parse().unwrap(),
                ) )
            },
            _ => panic!()
        }
    }

    queries_vec
}