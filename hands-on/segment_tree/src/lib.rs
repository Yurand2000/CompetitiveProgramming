use std::marker::PhantomData;

pub mod traits;
mod utils;

pub use traits::*;
use utils::*;

/// Lazy Segment Tree implementation.
/// 
/// It requires a data type `T` stored in the nodes that implements [`Clone`](Clone),
/// an operation `Op` which implements [`Monoid`](Monoid),
/// and a function `F` which implements [`UpdateFunction`](UpdateFunction),
/// [`ComposableFunction`](ComposableFunction) and [`Monoid`](Monoid).
/// 
/// The segment tree implementation operates for both queries and updates in
/// `Θ(log n)` time, where n is the number of leaves in the tree, and occupies
/// `Θ(n)` space.
#[derive(Debug)]
pub struct LazySegmentTree<T, Op, F>
    where T: Clone, Op: Monoid<Data = T>,
        F: UpdateFunction<Data = T> + ComposableFunction + Monoid + Clone
{
    values: Vec< TreeNode<T> >,
    updates: Vec< F >,
    max_range: usize,
    _op : PhantomData<Op>
}

impl<T, Op, F> LazySegmentTree<T, Op, F>
    where T: Clone, Op: Monoid<Data = T>, F: UpdateFunction<Data = T> + ComposableFunction + Monoid + Clone
{
    /// Creates a new `LazySegmentTree` given a vector of initial values for the leaves. It initializes the
    /// segment tree using the [`Semigroup`](Semigroup)'s provided operation.
    pub fn new(vec: Vec<T>) -> Self {
        let leafs_num = vec.len();
        let tree_size = 2 * leafs_num - 1;

        //initialize empty segment tree
        let mut tree = Self{
            values: vec![ TreeNode::identity::<Op>(); tree_size ],
            updates: vec![ F::identity(); tree_size ],
            max_range: leafs_num,
            _op: Default::default()
        };

        //compute segment tree
        tree.initialize_leafs(vec);
        for i in (0..tree.values.len()).rev() {
            tree.initialize_from_children(i);
        }

        tree
    }

    /// The leaves are copied into the segment tree.
    fn initialize_leafs(&mut self, leafs: Vec<T>) {
        let leafs_num = leafs.len();
        let tree_size = self.values.len();

        // the leafs must be copied from the leftmost to the rightmost
        // indipendently of their height. For segment trees they will
        // always be in the ending portion of the tree array, on the tree
        // they will be in both the last and the one before it. The leaves
        // array must be rotated by the number of leaves at the lowest level.
        let rotation = (1 << f32::log2(leafs_num as f32).ceil() as usize) - leafs_num;
        let array_it = leafs.into_iter().rev().cycle().skip(rotation);

        self.values.iter_mut().skip(tree_size - leafs_num).rev().zip(array_it)
            .for_each(|(vec, val)| { vec.data = val; });
    }

    /// Any inner node is initialized by combining the value of
    /// its children. Its range will span on both the children nodes.
    fn initialize_from_children(&mut self, index: usize)
    {
        let (node, l_child, r_child) = borrow_mut_node_and_mut_children(&mut self.values, index);
        if let (Some(lnode), Some(rnode)) = (l_child, r_child)
        {
            node.data = Op::combine(&lnode.data, &rnode.data);
            node.size = lnode.size + rnode.size;
        }
    }

    /// Query recursive function, must be started from the root node.
    fn query_rec(&mut self, (l, r): (isize, isize), index: usize) -> T {
        let node_size = self.values[index].size as isize;

        if total_overlap((l, r), node_size) {
            self.propagate_update_to_children(index);
            let new_value = self.apply_update(index);

            new_value
        }
        else if no_overlap((l, r), node_size) {
            Op::identity()
        }
        else /*partial overlap*/ {
            assert!(node_size > 1); //cannot have partial overlaps when the node has size 1.
            
            self.propagate_update_to_children(index);
            self.apply_update(index);

            //go recursively left and right
            let (l_range, r_range) = self.get_child_ranges((l, r), index);
            let left = self.query_rec(l_range, l_child(index));
            let right = self.query_rec(r_range, r_child(index));
            
            Op::combine(&left, &right)
        }
    }

    /// Update recursive function, must be started from the root node.
    fn update_rec(&mut self, (l, r): (isize, isize), index: usize, f: &F) {
        let node_size = self.values[index].size as isize;
        if total_overlap((l, r), node_size) {
            self.compose_update(index, f);
            self.propagate_update_to_children(index);
            self.apply_update(index);
        }
        else if no_overlap((l, r), node_size) {
            self.propagate_update_to_children(index);
            self.apply_update(index);
        }
        else /*partial overlap*/ {
            assert!(node_size > 1); //cannot have partial overlaps when the node has size 1.

            self.propagate_update_to_children(index);
            self.reset_update(index);

            //go recursively left and right
            let (l_range, r_range) = self.get_child_ranges((l, r), index);
            self.update_rec(l_range, l_child(index), f);
            self.update_rec(r_range, r_child(index), f);
            
            self.update_from_children(index);
        }
    }

    /// Compose the given update function to the one for the node given its index.
    fn compose_update(&mut self, index: usize, f: &F) {
        let update_node = &mut self.updates[index];
        *update_node = F::compose(f, update_node);
    }

    /// Apply any pending update and reset it to the identity.
    /// Return the newly computed value.
    fn apply_update(&mut self, index: usize) -> T {
        let update_node = &mut self.updates[index];
        let value_node = &mut self.values[index];
        let new_value = update_node.apply(&value_node.data, value_node.size);
        *update_node = F::identity();
        value_node.data = new_value.clone();

        new_value
    }

    /// Set the update for the node to the identity.
    fn reset_update(&mut self, index: usize) {
        let update_node = &mut self.updates[index];
        *update_node = F::identity();
    }

    /// Propagate any pending update to the children nodes.
    fn propagate_update_to_children(&mut self, index: usize) {
        let (node, lnode, rnode) = borrow_mut_node_and_mut_children(&mut self.updates, index);
        if let Some(lnode) = lnode {
            *lnode = F::compose(node, lnode);
        }
        if let Some(rnode) = rnode {
            *rnode = F::compose(node, rnode);
        }
    }

    /// Updates the value of an inner node by combining
    /// the answers of its children.
    fn update_from_children(&mut self, index: usize)
    {
        let left = &self.values[l_child(index)].data;
        let right = &self.values[r_child(index)].data;
        let new_value = Op::combine(left, right);
        let value_node = &mut self.values[index];
        value_node.data = new_value.clone();
    }

    /// Given an inner node and a range, compute the two
    /// sub-ranges for the node's children.
    fn get_child_ranges(&self, (l, r): (isize, isize), index: usize) -> ( (isize, isize), (isize, isize) )
    {
        let m = self.values[l_child(index)].size as isize - 1;
        let left = (l, m.min(r));
        let right = (0.max( l - (m + 1) ), r - (m + 1));
        (left, right)
    }
}

impl<T, Op, F> SegmentTree for LazySegmentTree<T, Op, F>
    where T: Clone, Op: Monoid<Data = T>, F: UpdateFunction<Data = T> + ComposableFunction + Monoid + Clone
{
    type Data = T;
    type UpdateFn = F;

    /// Performs a query on the given segment tree given a range.
    /// 
    /// It performs in `Θ(log n)` time, where n is the number of leafs of the tree.
    fn query(&mut self, (l, r): (isize, isize)) -> Self::Data {
        let max_range = self.max_range as isize;
        self.query_rec(
            ( l.max(0), r.min(max_range) ),
            0 //root index
        )
    }

    /// Performs an update on the given segment tree given a range and an update function.
    /// This operation is performed lazily, and updates are performed only when needed.
    /// 
    /// It performs in `Θ(log n)` time, where n is the number of leafs of the tree.
    fn update(&mut self, (l, r): (isize, isize), f: Self::UpdateFn)
    {
        let max_range = self.max_range as isize;
        self.update_rec(
            ( l.max(0), r.min(max_range) ),
            0, //root index
            &f
        )
    }
}

/// Each node of the segment tree contains
/// the data stored in the node and the number
/// of elements the sub-tree contains.
#[derive(Debug, Clone)]
struct TreeNode<T> {
    data: T,
    size: usize
}

impl<T> TreeNode<T> {
    fn identity<Op>() -> Self
        where Op: Monoid<Data = T> {
        Self{ data: Op::identity(), size: 1 }
    }
}

#[cfg(test)]
mod tests;

