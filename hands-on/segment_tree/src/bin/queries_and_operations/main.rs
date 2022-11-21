use segment_tree::*;

mod input_parse;
use input_parse::*;

#[derive(Debug, Clone)]
pub struct Operation(usize, usize, i32);
pub struct Query(isize, isize);

fn main() {
    let (array, operations, queries) = read_input_from_stdin();

    let result = queries_of_operations(array, operations, queries);

    print!("{}", result[0]);
    for i in 1..result.len() {
        print!(" {}", result[i]);
    }
}

fn queries_of_operations(array: Vec<i32>, operations: Vec<Operation>, queries: Vec<Query>) -> Vec<i32>
{
    let mut tree: LazySegmentTree<i32, Any, Add> =
        LazySegmentTree::new(vec![Any::identity(); operations.len()]);

    for query in queries.iter() {
        tree.update( (query.0 - 1, query.1 - 1), Add(1) );
    }

    let mut prefix_sum_inverse = vec![0; array.len()];
    let mut prec = 0;
    for (&elem, prefix_elem) in array.iter().zip(prefix_sum_inverse.iter_mut()) {
        *prefix_elem = elem - prec;
        prec = elem;
    }

    for op in 0..operations.len() {
        let op_count = tree.query((op as isize, op as isize));
        let mut op = operations[op].clone();
        op.2 = op.2 * op_count;

        if let Some(cell) = prefix_sum_inverse.get_mut(op.0 - 1) { *cell += op.2; }
        if let Some(cell) = prefix_sum_inverse.get_mut(op.1) { *cell -= op.2; }
    }

    let mut array = vec![0; prefix_sum_inverse.len()];
    let mut prec = 0;
    for i in 0..prefix_sum_inverse.len() {
        array[i] = prefix_sum_inverse[i] + prec;
        prec = array[i];
    }

    array
}

#[derive(Default)]
struct Any;

impl Semigroup for Any {
    type Data = i32;

    fn combine(left: &Self::Data, right: &Self::Data) -> Self::Data { if *left == 0 { *right } else { *left } }
}

impl Monoid for Any {
    fn identity() -> Self::Data { 0 }
}

#[derive(Clone)]
struct Add(i32);

impl UpdateFunction for Add {
    type Data = i32;

    fn apply(&self, a: &Self::Data, _size: usize) -> Self::Data {
        a + self.0
    }
}

impl Semigroup for Add {
    type Data = Self;

    fn combine(left: &Self, right: &Self) -> Self {
        Add( left.0 + right.0 )
    }
}

impl Monoid for Add {
    fn identity() -> Self::Data {
        Self(0)
    }
}

impl ComposableFunction for Add { }

#[cfg(test)]
mod tests;