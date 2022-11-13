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

    /// The semigroup's associative binary operation.
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
    /// The monoid's identity element.
    /// 
    /// Requires: `∀a ⇒ op(a, identity()) = op(identity(), a) = a`
    fn identity() -> Self::Data;
}
pub trait UpdateFunction {
    type Data: Clone;

    /// Require ∀(f, a = (val_a, size_a), b = (val_b, size_b)) ⇒ apply(f, op(a, b), size_a + size_b) = op(apply(f, a, size_a), apply(f, b, size_b))
    fn apply(&self, a: &Self::Data, size: usize) -> Self::Data;
}

pub trait ComposableFunction: UpdateFunction + Semigroup<Data = Self> + Sized {
    /// Require ∀(f,g,a) ⇒ apply(compose(f, g), a) = apply(f, apply(g, a))
    fn compose(left: &Self, right: &Self) -> Self {
        Self::op(left, right)
    }
}

pub trait SegmentTree {
    type Data: Clone;
    type UpdateFn: UpdateFunction<Data = Self::Data>;
    
    fn query(&mut self, range: (isize, isize)) -> Self::Data;
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