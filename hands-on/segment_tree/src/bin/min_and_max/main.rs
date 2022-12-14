use segment_tree::*;

mod input_parse;
use input_parse::*;

#[derive(Debug)]
pub enum Query {
    Update(isize, isize, i32),
    Max(isize, isize)
}

fn main() {
    let (array, queries) = read_input_from_stdin();

    let query_res = min_and_max(array, queries);

    for res in query_res.iter() {
        println!("{}", res);
    }
}

fn min_and_max(array: Vec<i32>, queries: Vec<Query>) -> Vec<i32>
{
    let mut tree: LazySegmentTree<i32, Max, Option<MinUpdate>> =
        LazySegmentTree::new(array);

    let mut query_results = Vec::new();
    for query in queries.iter()
    {
        match *query {
            Query::Update(l, r, val) => {
                let range = (l-1, r-1);
                tree.update(range, Some(MinUpdate(val)));
            },
            Query::Max(l, r) => {
                let range = (l-1, r-1);
                query_results.push( tree.query(range) );
            },
        }
    }

    query_results
}

/// Defines the operation that is performed when combining
/// two query ranges on the segment tree.
#[derive(Default)]
struct Max;

impl Semigroup for Max {
    type Data = i32;

    fn combine(left: &Self::Data, right: &Self::Data) -> Self::Data { *left.max(right) }
}

impl Monoid for Max {
    fn identity() -> Self::Data { i32::MIN }
}

/// Defines the operation that is performed when updating
/// a range on the segment tree.
#[derive(Clone)]
struct MinUpdate(i32);

impl UpdateFunction for MinUpdate {
    type Data = i32;

    fn apply(&self, a: &Self::Data, _size: usize) -> Self::Data {
        self.0.min(*a)
    }
}

impl Semigroup for MinUpdate {
    type Data = Self;

    fn combine(left: &Self, right: &Self) -> Self {
        MinUpdate( left.0.min(right.0) )
    }
}

impl ComposableFunction for MinUpdate { }

#[cfg(test)]
mod tests;