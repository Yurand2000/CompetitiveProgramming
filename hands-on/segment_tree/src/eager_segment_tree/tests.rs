use crate::eager_segment_tree::*;

#[test]
fn test() {
    let mut tree: EagerSegmentTree<i32, SumI32, PlusFn> = EagerSegmentTree::new(vec![10, 15, 20, 25]);

    assert_eq!(tree.query( (0, 1) ), 25);
    assert_eq!(tree.query( (1, 2) ), 35);
    assert_eq!(tree.query( (0, 3) ), 70);

    tree.update((1, 2), PlusFn(5));

    assert_eq!(tree.query( (0, 1) ), 30);
    assert_eq!(tree.query( (1, 2) ), 45);
    assert_eq!(tree.query( (0, 3) ), 80);
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

#[derive(Debug, Clone)]
struct PlusFn(i32);

impl UpdateFunction for PlusFn {
    type Data = i32;

    fn apply(&self, a: &Self::Data, size: usize) -> Self::Data {
        a + self.0 * size as i32
    }
}