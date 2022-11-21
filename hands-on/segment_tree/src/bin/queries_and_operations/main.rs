mod input_parse;
use input_parse::*;

pub struct Operation{
    left: usize,
    right: usize,
    value: i32
}
pub struct Query{
    left: usize,
    right: usize
}

fn main() {
    let (array, operations, queries) = read_input_from_stdin();

    let result = queries_of_operations(array, operations, queries);

    print!("{}", result[0]);
    for item in result.iter().skip(1) {
        print!(" {}", item);
    }
}

fn queries_of_operations(array: Vec<i32>, operations: Vec<Operation>, queries: Vec<Query>) -> Vec<i32>
{
    let operation_occurrencies =
        compute_ops_occurrencies(operations.len(), queries);

    compute_ops_executions(array, operations, operation_occurrencies)
}

/// Computes how many times each update operation must be performed
/// given the set of queries.
/// 
/// Given that update queries add the value 1 to a range of values,
/// it is possible to use the prefix sum inverse of the array of
/// operation occurrencies to perform this update in constant time
/// for each query.
/// 
/// This function performs in `Θ(m + k)`, where there are `k` queries
/// and `m` operations, because the operation array must be computed
/// from its inverted prefix sum and the queries must be performed.
fn compute_ops_occurrencies(operation_count: usize, queries: Vec<Query>) -> Vec<i32> {
    let mut ops_psum = vec![0; operation_count];

    for query in queries.iter() {
        let range = (query.left - 1, query.right - 1);
        prefix_sum_add_to_range(&mut ops_psum, range, 1);
    }

    array_to_prefix_sum(&ops_psum)
}

/// Given an array and a number of update operations with their occurrence,
/// the final array after all the updates is computed.
/// 
/// Since the operations perform a range update of values, it is possible
/// to exploit the prefix sum inverse of the array of values to perform
/// each update in constant time.
/// 
/// This function performs in `Θ(m + n)`, where there are `m` operations and
/// `n` values, because the values array must be computed from its inverted
/// prefix sum and the operations updates must be performed.
fn compute_ops_executions(array: Vec<i32>, operations: Vec<Operation>, ops_occurrencies: Vec<i32>) -> Vec<i32> {
    let mut psum = array_from_prefix_sum(&array);

    for (operation, occurrencies) in operations.iter().zip(ops_occurrencies.iter())
    {
        let range = (operation.left - 1, operation.right - 1);
        let value = operation.value * occurrencies;

        prefix_sum_add_to_range(&mut psum, range, value);
    }

    array_to_prefix_sum(&psum)
}

/// Computes the prefix sum array of the given vector.
/// 
/// This is the inverse function of
/// [`array_from_prefix_sum`](array_from_prefix_sum)
fn array_to_prefix_sum(input: &Vec<i32>) -> Vec<i32>
{
    let mut output = Vec::with_capacity(input.len());

    let mut prec = 0;
    for &elem in input.iter() {
        prec += elem;
        output.push(prec);
    }

    output
}

/// Computed the array from the given prefix sum vector.
/// Equally, given an array of values, produces a vector
/// of which its prefix sum is the starting vector.
/// 
/// This is the inverse function of
/// [`array_to_prefix_sum`](array_to_prefix_sum)
fn array_from_prefix_sum(input: &Vec<i32>) -> Vec<i32>
{
    let mut output = Vec::with_capacity(input.len());

    let mut prec = 0;
    for &elem in input.iter() {
        output.push(elem - prec);
        prec = elem;
    }

    output
}

/// Performs an update on a prefix sum inverse array, on which
/// each value update on a cell propagates to the rest of the array
/// when computing the prefix sum.
fn prefix_sum_add_to_range(vec: &mut [i32], (l, r): (usize, usize), value: i32)
{
    if let Some(cell) = vec.get_mut(l) {
        *cell += value;
    }

    if let Some(cell) = vec.get_mut(r + 1) {
        *cell -= value;
    }
}

#[cfg(test)]
mod tests;