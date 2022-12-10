mod input_parse;

#[derive(Clone, PartialEq, Eq)]
pub enum XmasLight {
    Red,
    White,
    Green,
    Unknown
}

fn main()
{
    let input = include_str!("./tests/input1.txt");
    let input_lines = &mut input.lines().map(|line| String::from(line));
    let lights = input_parse::read_input(input_lines);

    let (red, white, green) = solve(lights);
    println!("{} {} {}", red, white, green);
}

fn solve(lights: Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut white, mut green, mut k) = (0, 0, 0, 1);
    for light in lights.iter() {
        match light {
            XmasLight::Red => {
                red += 1;
            },
            XmasLight::White => {
                white += red;
            },
            XmasLight::Green => {
                green += white;
            }
            XmasLight::Unknown => {
                let (curr_red, curr_white, curr_green) = (red, white, green);
                red = 3 * curr_red + k;
                white = 3 * curr_white + curr_red;
                green = 3 * curr_green + curr_white;
                k = 3 * k;
            }
        }
    }

    (red, white, green)
}

fn bruteforce_solve(lights: Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut white, mut green) = (0, 0, 0);

    let mut no_unknown = lights.clone();
    let mut unknowns = 0;
    for light in no_unknown.iter_mut() {
        if *light == XmasLight::Unknown {
            *light = XmasLight::Red;
            unknowns += 1;
        } 
    }

    let combinations = 3u32.pow(unknowns);
    for i in 0..combinations {
        let mut modulus = i;
        let mut k = unknowns as i32 - 1;
        for (index, light) in lights.iter().enumerate() {
            if *light == XmasLight::Unknown {
                let color = modulus / 3u32.pow(k as u32);
                modulus = modulus % 3u32.pow(k as u32);
                k -= 1;
    
                match color {
                    0 => no_unknown[index] = XmasLight::Red,
                    1 => no_unknown[index] = XmasLight::White,
                    2 => no_unknown[index] = XmasLight::Green, 
                    v => panic!("{v}"),
                }
            }
        }

        let (new_red, new_white, new_green) = solve_simple(&no_unknown);
        red += new_red;
        white += new_white;
        green += new_green;
    }

    (red, white, green)
}

fn solve_simple(lights: &Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut white, mut green) = (0, 0, 0);
    for light in lights.iter() {
        match light {
            XmasLight::Red => {
                red += 1;
            },
            XmasLight::White => {
                white += red;
            },
            XmasLight::Green => {
                green += white;
            }
            XmasLight::Unknown => {
                panic!();
            }
        }
    }

    (red, white, green)
}

#[cfg(test)]
mod tests;