use std::io;
use segment_tree::*;

enum Query {
    Update(i32, i32, i32),
    Max(i32, i32)
}

fn main() {
    let (array, queries) = read_input();
    
}

fn read_input() -> (Vec<i32>, Vec<Query>) {
    let (size, queries) = read_size_from_io();
    let array = read_array_from_io(size);
    let queries = read_queries_from_io(queries);

    (array, queries)
}

fn read_size_from_io() -> (usize, usize)
{
    let stdin = io::stdin();
    let mut sizes_str = String::new();
    stdin.read_line(&mut sizes_str).unwrap();

    let mut sizes_str = sizes_str.trim().split_ascii_whitespace();
    let size = sizes_str.next().unwrap().parse().unwrap();
    let queries = sizes_str.next().unwrap().parse().unwrap();

    (size, queries)
}

fn read_array_from_io(size: usize) -> Vec<i32>
{
    let stdin = io::stdin();
    let mut array_str = String::new();
    stdin.read_line(&mut array_str).unwrap();

    let mut array_str = array_str.trim().split_ascii_whitespace();
    let mut array = Vec::with_capacity(size);
    for _ in 0..size {
        array.push(array_str.next().unwrap().parse().unwrap());
    }

    array
}

fn read_queries_from_io(queries: usize) -> Vec<Query>
{
    let stdin = io::stdin();
    let mut query_str = String::new();
    let mut queries_vec = Vec::with_capacity(queries);

    for _ in 0..queries
    {
        stdin.read_line(&mut query_str).unwrap();
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

        query_str.clear();
    }

    queries_vec
}