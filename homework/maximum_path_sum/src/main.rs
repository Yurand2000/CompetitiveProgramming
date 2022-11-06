#[derive(Debug)]
struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

fn leaf_max_sum(tree: &Tree) -> (i32, i32) {
    if tree.left.is_some() && tree.right.is_some() {
        let left = leaf_max_sum(tree.left.as_ref().unwrap());
        let right = leaf_max_sum(tree.right.as_ref().unwrap());

        (
            left.0.max(right.0) + tree.value,
            left.1.max(right.1).max(left.0 + right.0 + tree.value)
        )
    }
    else if tree.left.is_some() {
        let left = leaf_max_sum(tree.left.as_ref().unwrap());

        (
            left.0 + tree.value,
            i32::MIN.max(left.1)
        )        
    }
    else if tree.right.is_some() {
        let right = leaf_max_sum(tree.right.as_ref().unwrap());

        (
            right.0 + tree.value,
            i32::MIN.max(right.1)
        )        
    }
    else {
        (
            tree.value,
            i32::MIN
        )
    }
}

fn main() {
    //Example 1
    let tree = *Tree::from_iterator(&mut (vec! [
            (3, true, true),
                (4, true, true),
                    (-10, false, false),
                    (4, false, false),
                (5, false, false)
    ].into_iter().map(|(value, left, right)| -> TreeRep { 
        TreeRep::new(value, left, right)
    }))).unwrap();
    let max_sum = leaf_max_sum(&tree);
    let expected_max_sum = 16;
    assert_eq!(max_sum.1, expected_max_sum);

    //Example 2
    let tree = *Tree::from_iterator(&mut (vec! [
            (-15, true, true),
                (5, true, true),
                    (-8, true, true),
                        (2, false, false),
                        (-3, false, false),
                    (1, false, false),
                (6, true, true),
                    (3, false, false),
                    (9, false, true),
                        (0, true, true),
                            (4, false, false),
                            (-1, true, false),
                                (10, false, false)
    ].into_iter().map(|(value, left, right)| -> TreeRep { 
        TreeRep::new(value, left, right)
    }))).unwrap();
    let max_sum = leaf_max_sum(&tree);
    let expected_max_sum = 27;
    assert_eq!(max_sum.1, expected_max_sum);

    println!("Success!");
}

struct TreeRep {
    value: i32,
    left: bool,
    right: bool
}

impl TreeRep {
    fn new(value: i32, left: bool, right: bool) -> Self {
        TreeRep{ value, left, right }
    }
}

impl Tree {
    fn from_iterator<T>(rep: &mut T) -> Option<Box<Tree>>
    where T: Iterator<Item=TreeRep>
    {
        if let Some(node) = rep.next() {
            let mut tree = Tree{ value: node.value, left: None, right: None };
            if node.left {
                tree.left = Self::from_iterator(rep);
            }
            if node.right {
                tree.right = Self::from_iterator(rep);
            }

            Some(Box::new(tree))
        } else {
            None
        }
    }
}