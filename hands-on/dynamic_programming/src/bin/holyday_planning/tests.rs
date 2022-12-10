use crate::*;

#[test] fn test0() { run_test(include_str!("./tests/input0.txt"), include_str!("./tests/output0.txt")); }
#[test] fn test1() { run_test(include_str!("./tests/input1.txt"), include_str!("./tests/output1.txt")); }
#[test] fn test2() { run_test(include_str!("./tests/input2.txt"), include_str!("./tests/output2.txt")); }
#[test] fn test3() { run_test(include_str!("./tests/input3.txt"), include_str!("./tests/output3.txt")); }
#[test] fn test4() { run_test(include_str!("./tests/input4.txt"), include_str!("./tests/output4.txt")); }

fn run_test(in_file: &str, out_file: &str)
{
    let (mut cities, days) = parse_input(in_file);
    let expected_output = parse_output(out_file);

    preprocessing(&mut cities);   
    let generated_output = solve(cities, days);

    assert_eq!(expected_output, generated_output);
}

fn parse_input(in_file: &str) -> (Vec<City>, usize)
{
    input_parse::read_input(
        &mut in_file.lines().map(|line| String::from(line) )
    )
}

fn parse_output(out_file: &str) -> i32
{
    out_file.trim().parse().unwrap()
}

