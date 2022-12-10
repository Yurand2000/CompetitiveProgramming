use crate::*;

#[test] fn test0() { run_test(include_str!("./tests/input0.txt"), include_str!("./tests/output0.txt")); }
#[test] fn test1() { run_test(include_str!("./tests/input1.txt"), include_str!("./tests/output1.txt")); }
#[test] fn test2() { run_test(include_str!("./tests/input2.txt"), include_str!("./tests/output2.txt")); }
#[test] fn test3() { run_test(include_str!("./tests/input3.txt"), include_str!("./tests/output3.txt")); }
#[test] fn test4() { run_test(include_str!("./tests/input4.txt"), include_str!("./tests/output4.txt")); }
#[test] fn test5() { run_test(include_str!("./tests/input5.txt"), include_str!("./tests/output5.txt")); }
#[test] fn test6() { run_test(include_str!("./tests/input6.txt"), include_str!("./tests/output6.txt")); }

fn run_test(in_file: &str, out_file: &str)
{
    let lights = parse_input(in_file);
    let expected_output = parse_output(out_file);

    let (_red, _rw_combs, rwg_combs) = solve(lights);

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

