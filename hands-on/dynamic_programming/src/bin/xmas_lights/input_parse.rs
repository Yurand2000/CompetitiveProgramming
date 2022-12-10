use std::io::{self, BufReader, BufRead};

use crate::XmasLight;

#[inline(always)]
pub fn read_input_from_stdin() -> Vec<XmasLight> {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines().map(|next| next.unwrap() );
    read_input(&mut lines)
}

pub fn read_input<T>(input: &mut T) -> Vec<XmasLight>
    where T: Iterator<Item = String>
{
    let houses = read_size_from_io(input);
    let lights = read_colors_from_io(&mut input.next().unwrap().chars(), houses);

    lights
}

fn read_size_from_io<T>(input: &mut T) -> usize
    where T: Iterator<Item = String>
{
    let sizes_str = input.next().unwrap();

    let mut sizes_str = sizes_str.split_whitespace();
    let houses = sizes_str.next().unwrap().parse().unwrap();

    houses
}

fn read_colors_from_io<T>(input: &mut T, houses: usize) -> Vec<XmasLight>
    where T: Iterator<Item = char>
{
    let mut lights = Vec::with_capacity(houses);

    for _ in 0..houses {
        let color_char = input.next().unwrap();
        let color = match color_char {
            'R' => XmasLight::Red,
            'W' => XmasLight::White,
            'G' => XmasLight::Green,
            'X' => XmasLight::Unknown,
            _ => panic!(),
        };

        lights.push(color);
    }

    lights
}