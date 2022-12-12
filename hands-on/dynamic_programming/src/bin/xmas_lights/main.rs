mod input_parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmasLight {
    Red,
    White,
    Green,
    Unknown
}

fn main()
{
    let lights = input_parse::read_input_from_stdin();

    let (_red, _rw_combs, rwg_combs) = solve(&lights);
    println!("{}", rwg_combs);
}

fn solve(lights: &Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut rw_combs, mut rwg_combs, mut k) = (0, 0, 0, 1);
    for light in lights.iter() {
        match light {
            XmasLight::Red => {
                red += k;
            },
            XmasLight::White => {
                rw_combs += red;
            },
            XmasLight::Green => {
                rwg_combs += rw_combs;
            }
            XmasLight::Unknown => {
                let (curr_red, curr_rw_combs, curr_rwg_combs) = (red, rw_combs, rwg_combs);
                red = 3 * curr_red + k;
                rw_combs = 3 * curr_rw_combs + curr_red;
                rwg_combs = 3 * curr_rwg_combs + curr_rw_combs;
                k = 3 * k;
            }
        }
    }

    (red, rw_combs, rwg_combs)
}

#[allow(dead_code)]
fn solve_simple(lights: &Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut rw_combs, mut rwg_combs) = (0, 0, 0);
    for light in lights.iter() {
        match light {
            XmasLight::Red => {
                red += 1;
            },
            XmasLight::White => {
                rw_combs += red;
            },
            XmasLight::Green => {
                rwg_combs += rw_combs;
            }
            XmasLight::Unknown => {
                panic!("Simpler problem does not have unknown lights.");
            }
        }
    }

    (red, rw_combs, rwg_combs)
}

#[cfg(test)]
mod tests;