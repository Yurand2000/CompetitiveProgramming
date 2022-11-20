/// A [Semigroup](https://en.wikipedia.org/wiki/Semigroup) is a *set* with
/// an internal *associative binary operation*.
/// By implementing this trait, the *set* is represented by all the
/// possible values the implementor *type* can be, while the *binary operation*
/// must be provided by implementing the trait.
/// 
/// Note that the provided operation must be associative and that this check
/// cannot be enforced by the compiler.
pub trait Semigroup: Sized
{
    type Data: Clone;

    /// The `Semigroup`'s associative binary operation.
    /// 
    /// Requires: `∀(a,b,c) ⇒ op(a, op(b, c)) = op(op(a, b), c)`
    fn op(left: &Self::Data, right: &Self::Data) -> Self::Data;
}

/// A [Monoid](https://en.wikipedia.org/wiki/Monoid) is a *set* with
/// an internal *associative binary operation* and an *identity element*.
/// Is is here represented as an extension of the trait [`Semigroup`](Semigroup), with
/// the addition of the `identity` function which provided the identity
/// value for the provided *binary operation*.
/// 
/// Note that the provided operation must have an identity element and that
/// the `identity` function provides this element. Additionally bear in mind
/// that the compiler cannot enforce these properties.
pub trait Monoid: Semigroup
{
    /// The `Monoid`'s identity element.
    /// 
    /// Requires: `∀a ⇒ op(a, identity()) = op(identity(), a) = a`
    fn identity() -> Self::Data;
}

/// The trait [`UpdateFunction`] specifies functions which are used to
/// update segment trees. It performs an update for a specific
/// range of elements on the segment tree, given the query value at that
/// node and the size of the sub-tree (number of leafs).
/// 
/// If the segment tree is eager, the function is applied only to the
/// leaves, and size will always be 1. The value of the inner nodes is
/// computed using the combine operation on the tree's query operation.
/// If the segment tree is lazy, the function may be applied to inner
/// nodes too, or stored for later lazy updates when required.
/// 
/// Note that the provided operation must be distributive with the `op`
/// operation provided by `Semigroup` and that this check cannot be
/// enforced by the compiler.
pub trait UpdateFunction {
    type Data: Clone;

    /// The `UpdateFunction`'s application.
    /// 
    /// Require ∀(f, a = (val_a, size_a), b = (val_b, size_b)) ⇒ apply(f, op(a, b), size_a + size_b) = op(apply(f, a, size_a), apply(f, b, size_b))
    fn apply(&self, a: &Self::Data, size: usize) -> Self::Data;
}

/// The trait [`ComposableFunction`] specifies how to compose two
/// [`UpdateFunction`](UpdateFunction)s into a single update function.
/// The definition is the same as that of a [`Semigroup`](Semigroup),
/// by requiring also that the implementor type is an
/// [`UpdateFunction`](UpdateFunction).
/// 
/// This functionality is exploited in lazy segment trees to combine
/// multiple updates onto the same range into one single update, before
/// propagating it to the leaves.
/// 
/// Note that the provided operation must be distributive with the `apply` operation
/// provided by `UpdateFunction` and that this check cannot be enforced by the compiler.
pub trait ComposableFunction: UpdateFunction + Semigroup<Data = Self> + Sized {
    /// The `ComposableFunction`'s composition.
    /// 
    /// Require ∀(f,g,a) ⇒ apply(compose(f, g), a) = apply(f, apply(g, a))
    fn compose(left: &Self, right: &Self) -> Self {
        Self::op(left, right)
    }
}

/// The trait [`Segment Tree`] specifies the functions and constraints a
/// segment tree implementation must provide.
pub trait SegmentTree {
    type Data: Clone;
    type UpdateFn: UpdateFunction<Data = Self::Data>;
    
    /// Performs a query on the segment tree given a range.
    fn query(&mut self, range: (isize, isize)) -> Self::Data;

    /// Performs an update on the segment tree given a range and
    /// an update function.
    fn update(&mut self, range: (isize, isize), function: Self::UpdateFn);
}

/// Implementation of traits for generic Option types
/// Implementation of Semigroup for generic Option types
impl<T: Semigroup<Data = T> + Clone> Semigroup for Option<T>{
    type Data = Self;

    fn op(left: &Self, right: &Self) -> Self
    {
        match (left, right) {
            (Some(ref a), Some(ref b)) => Some(T::op(a, b)),
            (None, _)                  => right.clone(),
            (_, None)                  => left.clone()
        }
    }
}

/// Implementation of Monoid for generic Option types
impl<T: Semigroup<Data = T> + Clone> Monoid for Option<T> {
    fn identity() -> Self {
        None
    }
}

/// Implementation of UpdateFunction for generic Option types
impl<F: UpdateFunction> UpdateFunction for Option<F> {
    type Data = F::Data;

    fn apply(&self, a: &Self::Data, size: usize) -> Self::Data
    {
        match self {
            Some(ref f) => F::apply(f, a, size),
            None => a.clone(),
        }
    }
}

/// Implementation of ComposableFunction for generic Option types
impl<F: ComposableFunction + Clone> ComposableFunction for Option<F> { }