#[inline(always)] pub fn l_child(node: usize) -> usize { (2 * node) + 1 }
#[inline(always)] pub fn r_child(node: usize) -> usize { (2 * node) + 2 }

#[inline(always)]
pub fn borrow_mut_node_and_mut_children<T>(vec: &mut Vec<T>, index: usize)
    -> (&mut T, Option<&mut T>, Option<&mut T>)
{
    let len = vec.len();
    let (left, right) = vec.split_at_mut( index + 1 );
    let left_len = left.len();
    let node = left.last_mut().unwrap();

    let (lnode, rnode) =
    if l_child(index) >= len {
        (None, None)
    } else {
        let (left, right) = right.split_at_mut( l_child(index) + 1 - left_len );
        (left.last_mut(), right.first_mut())
    };

    (node, lnode, rnode)
}

#[inline(always)] pub fn total_overlap((l, r): (isize, isize), node_size: isize) -> bool { (l, r + 1) == (0, node_size) }
#[inline(always)] pub fn no_overlap((l, r): (isize, isize), node_size: isize) -> bool { l >= node_size || r < 0 }