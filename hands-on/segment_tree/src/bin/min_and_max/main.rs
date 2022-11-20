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

    let mut tree: LazySegmentTree<i32, Max, MinUpdate> =
        LazySegmentTree::new(array);

    for query in queries.iter()
    {
        match *query {
            Query::Update(l, r, val) => tree.update((l-1, r-1), MinUpdate(val)),
            Query::Max(l, r) => {
                let value = tree.query((l-1, r-1));
                println!("{}", value);
            },
        }
    }
}

#[derive(Default)]
struct Max;

impl Semigroup for Max {
    type Data = i32;

    fn op(left: &Self::Data, right: &Self::Data) -> Self::Data { *left.max(right) }
}

impl Monoid for Max {
    fn identity() -> Self::Data { i32::MIN }
}

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

    fn op(left: &Self, right: &Self) -> Self {
        MinUpdate( left.0.min(right.0) )
    }
}

impl Monoid for MinUpdate {
    fn identity() -> Self::Data {
        Self(i32::MAX)
    }
}

impl ComposableFunction for MinUpdate { }

#[cfg(test)]
mod tests;