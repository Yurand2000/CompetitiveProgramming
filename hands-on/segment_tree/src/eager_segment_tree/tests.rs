use crate::eager_segment_tree::*;

#[test]
fn test() {
    let mut tree: EagerSegmentTree<SumI32> = EagerSegmentTree::new(
        [10, 15, 20, 25]
            .into_iter().map(|v| { SumI32::from(v) }).collect()
    );

    assert_eq!(tree.query( (0, 1) ), SumI32(25));
    assert_eq!(tree.query( (1, 2) ), SumI32(35));
    assert_eq!(tree.query( (0, 3) ), SumI32(70));

    tree.update((1, 2), PlusFn(5));

    assert_eq!(tree.query( (0, 1) ), SumI32(30));
    assert_eq!(tree.query( (1, 2) ), SumI32(45));
    assert_eq!(tree.query( (0, 3) ), SumI32(80));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SumI32(i32);

impl From<i32> for SumI32 {
    fn from(num: i32) -> Self {
        SumI32(num)
    }
}

impl Semigroup for SumI32 {
    fn op(left: &Self, right: &Self) -> Self { SumI32(left.0 + right.0) }
}

impl Monoid for SumI32 {
    fn identity() -> Self { SumI32(0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PlusFn(i32);

impl UpdateFunction for PlusFn {
    type NodeData = SumI32;

    fn apply(&self, a: &Self::NodeData, size: usize) -> Self::NodeData {
        SumI32(a.0 + self.0 * size as i32)
    }
}