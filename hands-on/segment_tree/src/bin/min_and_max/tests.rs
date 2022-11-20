use crate::*;

#[test] fn test0() { run_test(include_str!("./tests/input0.txt"), include_str!("./tests/output0.txt")); }
#[test] fn test1() { run_test(include_str!("./tests/input1.txt"), include_str!("./tests/output1.txt")); }
#[test] fn test2() { run_test(include_str!("./tests/input2.txt"), include_str!("./tests/output2.txt")); }
#[test] fn test3() { run_test(include_str!("./tests/input3.txt"), include_str!("./tests/output3.txt")); }
#[test] fn test4() { run_test(include_str!("./tests/input4.txt"), include_str!("./tests/output4.txt")); }
#[test] fn test5() { run_test(include_str!("./tests/input5.txt"), include_str!("./tests/output5.txt")); }
#[test] fn test6() { run_test(include_str!("./tests/input6.txt"), include_str!("./tests/output6.txt")); }
#[test] fn test7() { run_test(include_str!("./tests/input7.txt"), include_str!("./tests/output7.txt")); }
#[test] fn test8() { run_test(include_str!("./tests/input8.txt"), include_str!("./tests/output8.txt")); }
#[test] fn test9() { run_test(include_str!("./tests/input9.txt"), include_str!("./tests/output9.txt")); }
#[test] fn test10() { run_test(include_str!("./tests/input10.txt"), include_str!("./tests/output10.txt")); }

fn run_test(in_file: &str, out_file: &str)
{
    let (array, queries) = parse_input(in_file);
    let expected_output = parse_output(out_file);

    let generated_output = min_and_max(array, queries);

    assert_eq!(expected_output, generated_output);
}

fn parse_input(in_file: &str) -> (Vec<i32>, Vec<Query>)
{
    input_parse::read_input(
        &mut in_file.lines().map(|line| String::from(line) )
    )
}

fn parse_output(out_file: &str) -> Vec<i32>
{
    let mut output = Vec::new();
    for line in out_file.lines() {
        output.push(line.trim().parse().unwrap());
    }

    output
}

