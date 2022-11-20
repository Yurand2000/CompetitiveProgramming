use std::marker::PhantomData;

use crate::traits::*;
use crate::utils::*;

#[derive(Clone, Debug)]
struct LazySegmentTreeNode<T> {
    data: T,
    size: usize
}

pub struct LazySegmentTree<T, Op, F>
    where T: Clone, Op: Default + Monoid<Data = T>,
        F: UpdateFunction<Data = T> + ComposableFunction + Monoid
{
    values: Vec< LazySegmentTreeNode<T> >,
    updates: Vec< F >,
    max_range: usize,
    _op : PhantomData<Op>
}

impl<T, Op, F> LazySegmentTree<T, Op, F>
    where T: Clone, Op: Default + Monoid<Data = T>, F: UpdateFunction<Data = T> + ComposableFunction + Monoid + Clone
{
    pub fn new(vec: Vec<T>) -> Self {
        let array_len = vec.len();
        let tree_size = 2 * array_len - 1;
        let mut values = vec![LazySegmentTreeNode { data: Op::identity(), size: 1 }; tree_size];
        let updates = vec![F::identity(); tree_size];

        //copy input array
        let rotation = (1 << f32::log2(array_len as f32).ceil() as usize) - array_len;
        let array_it = vec.into_iter().rev().cycle().skip(rotation);

        values.iter_mut().skip(tree_size - array_len).rev().zip(array_it)
            .for_each(|(vec, val)| { vec.data = val; });

        //compute segment tree
        let mut tree = Self { values, updates, max_range: array_len, _op: Default::default() };
        for i in (0..tree.values.len()).rev() {
            tree.value_from_children(i);
        }

        tree
    }

    fn value_from_children(&mut self, index: usize)
    {
        let (node, lnode, rnode) = borrow_mut_node_and_children(&mut self.values, index);
        match (lnode, rnode) {
            (Some(lnode), Some(rnode)) => {
                node.data = Op::op(&lnode.data, &rnode.data);
                node.size = lnode.size + rnode.size;
            },
            _ => {},
        }
    }

    fn query_index(&mut self, (l, r): (isize, isize), index: usize) -> T {
        let node_size = self.values[index].size as isize;

        if (l, r + 1) == (0, node_size) {
            //total overlap
            self.propagate_update(index);
            let new_value = self.apply_update(index);

            new_value
        }
        else if l >= node_size || r < 0
        {
            //no overlap
            Op::identity()
        }
        else
        {
            //partial overlap
            assert!(node_size > 1);
            
            self.propagate_update(index);
            self.apply_update(index);

            //go recursively left and right
            let (l_range, r_range) = self.get_child_ranges((l, r), index);
            let left = self.query_index(l_range, lchild(index));
            let right = self.query_index(r_range, rchild(index));
            
            Op::op(&left, &right)
        }
    }

    fn update_index(&mut self, (l, r): (isize, isize), index: usize, f: &F) {
        let node_size = self.values[index].size as isize;
        if (l, r + 1) == (0, node_size) {
            //total overlap
            self.compose_update(index, f);
            self.propagate_update(index);
            self.apply_update(index);
        }
        else if l >= node_size || r < 0
        {
            //no overlap
            self.propagate_update(index);
            self.apply_update(index);
        }
        else
        {            
            //partial overlap
            assert!(node_size > 1);

            self.propagate_update(index);
            self.reset_update(index);

            //go recursively left and right
            let (l_range, r_range) = self.get_child_ranges((l, r), index);
            self.update_index(l_range, lchild(index), f);
            self.update_index(r_range, rchild(index), f);
            
            self.update_from_children(index);
        }
    }

    fn compose_update(&mut self, index: usize, f: &F) {
        let update_node = &mut self.updates[index];
        *update_node = F::compose(f, update_node);
    }

    fn apply_update(&mut self, index: usize) -> T {
        let update_node = &mut self.updates[index];
        let value_node = &mut self.values[index];
        let new_value = update_node.apply(&value_node.data, value_node.size);
        *update_node = F::identity();
        value_node.data = new_value.clone();

        new_value
    }

    fn reset_update(&mut self, index: usize) {
        let update_node = &mut self.updates[index];
        *update_node = F::identity();
    }

    fn propagate_update(&mut self, index: usize) {
        let (node, lnode, rnode) = borrow_mut_node_and_children(&mut self.updates, index);
        if let Some(lnode) = lnode {
            *lnode = F::compose(node, lnode);
        }
        if let Some(rnode) = rnode {
            *rnode = F::compose(node, rnode);
        }
    }

    fn update_from_recursive(&mut self, index: usize, left: T, right: T) -> T
    {
        let new_value = Op::op(&left, &right);
        let value_node = &mut self.values[index];
        value_node.data = new_value.clone();

        new_value
    }

    fn update_from_children(&mut self, index: usize)
    {
        self.update_from_recursive(
            index,
            self.values[lchild(index)].data.clone(),
            self.values[rchild(index)].data.clone()
        );
    }

    fn get_child_ranges(&self, (l, r): (isize, isize), index: usize) -> ( (isize, isize), (isize, isize) )
    {
        let m = self.values[lchild(index)].size as isize - 1;
        let left = (l, m.min(r));
        let right = (0.max( l - (m + 1) ), r - (m + 1));
        (left, right)
    }
}

impl<T, Op, F> SegmentTree for LazySegmentTree<T, Op, F>
    where T: Clone, Op: Default + Monoid<Data = T>, F: UpdateFunction<Data = T> + ComposableFunction + Monoid + Clone
{
    type Data = T;
    type UpdateFn = F;

    fn query(&mut self, (l, r): (isize, isize)) -> Self::Data {
        let max_range = self.max_range as isize;
        if l > r || l > max_range {
            Op::identity()
        } else if r > max_range - 1 {
            self.query_index((l, max_range), 0 )
        } else {
            self.query_index((l, r), 0)
        }
    }

    fn update(&mut self, (l, r): (isize, isize), f: Self::UpdateFn)
    {
        let max_range = self.max_range as isize;
        if l > r || l > max_range {
            //no update
        } else if r > max_range - 1 {
            self.update_index((l, max_range), 0, &f);
        } else {
            self.update_index((l, r), 0, &f);
        }
    }
}

#[cfg(test)]
mod tests;