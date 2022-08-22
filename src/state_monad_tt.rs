//! Functional Core/ Imperative Shell

/// Functor types are typically represented as an object with a .map() method
/// that maps from inputs to outputs while preserving structure.
/// In practice, “preserving structure” means that the return value is the same type of functor
/// (though values inside the container may be a different type).
/// Ref: https://medium.com/javascript-scene/functors-categories-61e031bac53f
/// Monad is a type of functor

// Pointed: A wrapper type that can wrap and unwrap a value T
// Self: Sized means this trait can only be impl by known size structures
trait Pointed
where
    Self: Sized,
{
    type Unit;
    fn of(unit: Self::Unit) -> Self;
    fn unwrap(self) -> Self::Unit;
}

struct Identity<T>(T);
impl<T> Pointed for Identity<T> {
    type Unit = T;
    fn of(unit: Self::Unit) -> Self {
        Identity(unit)
    }
    fn unwrap(self) -> Self::Unit {
        self.0
    }
}

// Functor has `of`, `unwrap` as part of its interface
trait Functor: Pointed {
    // `map` takes a function `F` and return a functor `B` which is transfered by `F`
    fn map<B, F>(self, f: F) -> B
    where
        B: Functor,
        F: Fn(Self::Unit) -> B::Unit,
    {
        B::of(f(self.unwrap()))
    }
}

trait Monad: Functor {
    fn chain<M, F>(self, f: F) -> M
    where
        M: Monad,
        F: Fn(Self::Unit) -> M,
        {
            f(self.unwrap())
        }
}