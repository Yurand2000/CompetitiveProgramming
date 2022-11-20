use crate::*;

#[test] fn test0() { run_test(include_str!("./tests/input0.txt"), include_str!("./tests/output0.txt")); }
#[test] fn test1() { run_test(include_str!("./tests/input1.txt"), include_str!("./tests/output1.txt")); }
#[test] fn test2() { run_test(include_str!("./tests/input2.txt"), include_str!("./tests/output2.txt")); }

fn run_test(in_file: &str, out_file: &str)
{
    let (array, operations, queries) = parse_input(in_file);
    let expected_output = parse_output(out_file);

    let generated_output = queries_of_operations(array, operations, queries);

    assert_eq!(expected_output, generated_output);
}

fn parse_input(in_file: &str) -> (Vec<i32>, Vec<Operation>, Vec<Query>)
{
    input_parse::read_input(
        &mut in_file.lines().map(|line| String::from(line) )
    )
}

fn parse_output(out_file: &str) -> Vec<i32>
{
    let mut output = Vec::new();
    for line in out_file.split_whitespace() {
        output.push(line.trim().parse().unwrap());
    }

    output
}

