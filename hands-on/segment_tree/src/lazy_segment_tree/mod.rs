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
        values.iter_mut().rev().zip(vec.into_iter().rev())
            .map(|(vec, val)| { vec.data = val; })
        .count();

        //compute segment tree
        for i in (0..(tree_size-array_len)).rev()
        {
            values[i].data = Op::op(
                &values[ lchild(i) ].data,
                &values[ rchild(i) ].data
            );

            values[i].size =
                values[ lchild(i) ].size +
                values[ rchild(i) ].size;
        }

        Self { values, updates, max_range: array_len, _op: Default::default() }
    }

    fn query_index(&mut self, (l, r): (isize, isize), index: usize) -> T {
        let node_size = self.values[index].size as isize;

        if (l, r + 1) == (0, node_size) {
            //total overlap
            let node = &mut self.values[index];
            node.data = self.updates[index].apply(&node.data, node.size);

            self.propagate_update(index);

            self.values[index].data.clone()
        }
        else if l >= node_size || r < 0
        {
            //no overlap
            Op::identity()
        }
        else
        {
            assert!(node_size > 1);
            
            //partial overlap
            self.propagate_update(index);

            let m = div_ceil(node_size, 2) - 1;
            let left = self.query_index((l, m), lchild(index));
            let right = self.query_index((0, r - (m + 1)), rchild(index));
            
            Op::op(&left, &right)
        }
    }

    fn update_index(&mut self, (l, r): (isize, isize), index: usize, f: &F) {
        let node_size = self.values[index].size as isize;
        if (l, r + 1) == (0, node_size) {
            //total overlap
            let node = &mut self.updates[index];
            *node = F::compose(f, node);
        }
        else if l >= node_size || r < 0
        {
            //no overlap
            //no update
        }
        else
        {
            assert!(node_size > 1);
            
            //partial overlap
            let node = &mut self.values[index];
            node.data = f.apply(&node.data, (r - l + 1).try_into().unwrap());

            let m = div_ceil(node_size, 2) - 1;
            self.update_index((l, m), lchild(index), f);
            self.update_index((0, r - (m + 1)), rchild(index), f);
        }
    }

    fn propagate_update(&mut self, index: usize) {        
        let (node, lnode, rnode) = borrow_mut_node_and_children(&mut self.updates, index);
        if let Some(lnode) = lnode {
            *lnode = F::compose(node, lnode);
        }
        if let Some(rnode) = rnode {
            *rnode = F::compose(node, rnode);
        }
        *node = F::identity();
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
            self.update_index((l, max_range), 0, &f)
        } else {
            self.update_index((l, r), 0, &f)
        }
    }
}

#[cfg(test)]
mod tests;