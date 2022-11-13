use crate::traits::*;
use crate::utils::*;

#[derive(Clone)]
struct EagerSegmentTreeNode<T> {
    data: T,
    size: usize
}

pub struct EagerSegmentTree<T>
    where T: Monoid {
    values: Vec< EagerSegmentTreeNode<T> >,
    max_range: usize
}

impl<T> EagerSegmentTree<T>
    where T: Monoid
{
    pub fn new(vec: Vec<T>) -> Self {
        let array_len = vec.len();
        let tree_size = 2 * array_len - 1;
        let mut values = vec![EagerSegmentTreeNode { data: T::identity(), size: 1 }; tree_size];

        //copy input array
        values.iter_mut().rev().zip(vec.into_iter().rev())
            .map(|(vec, val)| { vec.data = val; })
        .count();

        //compute segment tree
        for i in (0..(tree_size-array_len)).rev()
        {
            values[i].data = T::op(
                &values[ lchild(i) ].data,
                &values[ rchild(i) ].data
            );

            values[i].size =
                values[ lchild(i) ].size +
                values[ rchild(i) ].size;
        }

        Self { values, max_range: array_len }
    }

    pub fn query(&self, range: (isize, isize)) -> T {
        let (l, r) = range;
        let max_range = self.max_range as isize;
        if l > r || l > max_range {
            T::identity()
        } else if r > max_range - 1 {
            self.query_index((l, max_range), 0 )
        } else {
            self.query_index(range, 0)
        }
    }

    fn query_index(&self, (l, r): (isize, isize), index: usize) -> T {
        let node_size = self.values[index].size as isize;

        if (l, r + 1) == (0, node_size) {
            //total overlap
            self.values[index].data.clone()
        }
        else if l >= node_size || r < 0
        {
            //no overlap
            T::identity()
        }
        else
        {
            assert!(node_size > 1);
            
            //partial overlap
            let m = div_ceil(node_size, 2) - 1;
            let left = self.query_index((l, m), lchild(index));
            let right = self.query_index((0, r - (m + 1)), rchild(index));

            T::op(&left, &right)
        }
    }

    pub fn update<F>(&mut self, (l, r): (usize, usize), f: F) 
        where F: UpdateFunction<NodeData = T>
    {
        for i in l..=r.min(self.max_range - 1) {
            self.update_single( self.leaf_index(i), &f);
        }
    }

    fn update_single<F>(&mut self, index: usize, f: &F)
        where F: UpdateFunction<NodeData = T>
    {
        self.values[index].data = f.apply(
            &self.values[index].data,
            self.values[index].size
        );

        let mut curr = index;
        while curr > 0 {
            curr = parent(curr);
            self.recompute_node(curr);
        }
    }

    #[inline] fn leaf_index(&self, index: usize) -> usize {
        leaf_index(self.values.len(), self.max_range, index)
    }

    #[inline] fn recompute_node(&mut self, index: usize) {
        let left = &self.values[ lchild(index) ].data;
        let right = &self.values[ rchild(index) ].data;

        self.values[index].data = T::op(left, right);
    }
}

#[cfg(test)]
mod tests;