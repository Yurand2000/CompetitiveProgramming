use std::io::{self, BufReader, BufRead};
use crate::*;

#[inline(always)]
pub fn read_input_from_stdin() -> (Vec<City>, usize) {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines().map(|next| next.unwrap() );
    read_input(&mut lines)
}

pub fn read_input<T>(input: &mut T) -> (Vec<City>, usize)
    where T: Iterator<Item = String>
{
    let (cities_count, days) = read_sizes_from_io(input);
    let cities = read_cities_from_io(input, cities_count, days);

    (cities, days)
}

fn read_sizes_from_io<T>(input: &mut T) -> (usize, usize)
    where T: Iterator<Item = String>
{
    let sizes_str = input.next().unwrap();

    let mut sizes_str = sizes_str.split_whitespace();
    let cities_count = sizes_str.next().unwrap().parse().unwrap();
    let days = sizes_str.next().unwrap().parse().unwrap();

    (cities_count, days)
}

fn read_cities_from_io<T>(input: &mut T, cities_count: usize, days: usize) -> Vec<City>
    where T: Iterator<Item = String>
{
    let mut cities = Vec::new();
    for _ in 0..cities_count  {
        cities.push( read_city_from_io(input, days) );
    }

    cities
}

fn read_city_from_io<T>(input: &mut T, days: usize) -> City
    where T: Iterator<Item = String>
{
    let array_str = input.next().unwrap();

    let mut array_str = array_str.split_whitespace();
    let mut city = Vec::with_capacity(days);
    for _ in 0..days {
        city.push(array_str.next().unwrap().parse().unwrap());
    }

    City { attractions: city }
}