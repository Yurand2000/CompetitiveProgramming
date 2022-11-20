use crate::lazy_segment_tree::*;

#[test]
fn test() {
    let mut tree: LazySegmentTree<i32, SumI32, PlusFn> = LazySegmentTree::new(vec![10, 15, 20, 25]);

    assert_eq!(tree.query( (0, 1) ), 25);
    assert_eq!(tree.query( (1, 2) ), 35);
    assert_eq!(tree.query( (0, 3) ), 70);

    tree.update((1, 2), PlusFn(5));

    assert_eq!(tree.query( (0, 1) ), 30);
    assert_eq!(tree.query( (1, 2) ), 45);
    assert_eq!(tree.query( (0, 3) ), 80);

    tree.update((0, 1), PlusFn(10));

    assert_eq!(tree.query( (0, 1) ), 50);
    assert_eq!(tree.query( (1, 2) ), 55);
    assert_eq!(tree.query( (0, 3) ), 100);

    tree.update((0, 1), PlusFn(5));
    tree.update((1, 3), PlusFn(10));

    assert_eq!(tree.query( (0, 1) ), 70);
    assert_eq!(tree.query( (1, 2) ), 80);
    assert_eq!(tree.query( (0, 3) ), 140);
}

#[derive(Debug, Default)]
struct SumI32;

impl Semigroup for SumI32 {
    type Data = i32;

    fn op(left: &Self::Data, right: &Self::Data) -> Self::Data { left + right }
}

impl Monoid for SumI32 {
    fn identity() -> Self::Data { 0 }
}

#[derive(Debug, Clone, Copy)]
struct PlusFn(i32);

impl UpdateFunction for PlusFn {
    type Data = i32;

    fn apply(&self, a: &Self::Data, size: usize) -> Self::Data {
        a + self.0 * size as i32
    }
}

impl Semigroup for PlusFn {
    type Data = Self;

    fn op(left: &Self, right: &Self) -> Self {
        Self(left.0 + right.0)
    }
}

impl Monoid for PlusFn {
    fn identity() -> Self::Data {
        Self(0)
    }
}

impl ComposableFunction for PlusFn { }