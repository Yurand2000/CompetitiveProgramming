use rand::{prelude::Distribution, distributions::Standard, Rng};

use crate::*;

#[test] fn test0() { run_test(include_str!("./tests/input0.txt"), include_str!("./tests/output0.txt")); }
#[test] fn test1() { run_test(include_str!("./tests/input1.txt"), include_str!("./tests/output1.txt")); }
#[test] fn test2() { run_test(include_str!("./tests/input2.txt"), include_str!("./tests/output2.txt")); }
#[test] fn test3() { run_test(include_str!("./tests/input3.txt"), include_str!("./tests/output3.txt")); }
#[test] fn test4() { run_test(include_str!("./tests/input4.txt"), include_str!("./tests/output4.txt")); }
#[test] fn test5() { run_test(include_str!("./tests/input5.txt"), include_str!("./tests/output5.txt")); }
#[test] fn test6() { run_test(include_str!("./tests/input6.txt"), include_str!("./tests/output6.txt")); }

#[test]
#[ignore]
fn stress_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let lights = random_lights( rng.gen_range(0..20) );

        assert_eq!(solve(&lights), bruteforce_solve(&lights), "{:?}", &lights);
    }
}

fn run_test(in_file: &str, out_file: &str)
{
    let lights = parse_input(in_file);
    let expected_output = parse_output(out_file);

    let (_red, _rw_combs, rwg_combs) = solve(&lights);

    assert_eq!(expected_output, rwg_combs);
}

fn parse_input(in_file: &str) -> Vec<XmasLight>
{
    input_parse::read_input(
        &mut in_file.lines().map(|line| String::from(line) )
    )
}

fn parse_output(out_file: &str) -> i32
{
    out_file.trim().parse().unwrap()
}

fn random_lights(n: usize) -> Vec<XmasLight>
{
    let mut lights = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        lights.push( rng.gen() );
    }

    lights
}

pub fn bruteforce_solve(lights: &Vec<XmasLight>) -> (i32, i32, i32)
{
    let (mut red, mut rw_combs, mut rwg_combs) = (0, 0, 0);

    let unknown_lights = lights.iter().filter(|&l| *l == XmasLight::Unknown ).count() as i32;

    let combinations = 3u32.pow(unknown_lights as u32);
    for n in 0..combinations {

        let lights = compute_nth_combination(lights, n, unknown_lights);
        let (new_red, new_rws, new_rwgs) = solve_simple(&lights);
        
        (red, rw_combs, rwg_combs) = (red + new_red, rw_combs + new_rws, rwg_combs + new_rwgs);
    }

    (red, rw_combs, rwg_combs)
}

fn compute_nth_combination(lights: &Vec<XmasLight>, n: u32, unknown_lights: i32) -> Vec<XmasLight>
{
    let mut new_lights = Vec::with_capacity(lights.len());

    let mut modulus = n;
    let mut k = unknown_lights - 1;
    for light in lights.iter() {
        if *light == XmasLight::Unknown {
            let color = modulus / 3u32.pow(k as u32);
            modulus = modulus % 3u32.pow(k as u32);
            k -= 1;

            match color {
                0 => new_lights.push(XmasLight::Red),
                1 => new_lights.push(XmasLight::White),
                _ => new_lights.push(XmasLight::Green),
            }
        } else {
            new_lights.push( light.clone() );
        }
    }

    new_lights
}

impl Distribution<XmasLight> for Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> XmasLight {
        match rng.gen_range(0..=3) {
            0 => XmasLight::Red,
            1 => XmasLight::White,
            2 => XmasLight::Green,
            _ => XmasLight::Unknown,
        }
    }
}   
