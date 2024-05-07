use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either<L: Debug + Clone, R: Debug + Clone> {
    Left(L),
    Right(R),
}

impl<L: Debug + Clone, R: Debug + Clone> Either<L, R> {
    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Either::Right(_) => true,
            _ => false,
        }
    }

    pub fn is<T>(&self) -> bool
    where
        T: Debug + Clone,
    {
        match self {
            Either::Left(_) => std::any::type_name::<T>() == std::any::type_name::<L>(),
            Either::Right(_) => std::any::type_name::<T>() == std::any::type_name::<R>(),
        }
    }

    pub fn left(&self) -> Option<&L> {
        match self {
            Either::Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Either::Right(r) => Some(r),
            _ => None,
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut L> {
        match self {
            Either::Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut R> {
        match self {
            Either::Right(r) => Some(r),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_left() {
        let e: Either<i32, ()> = Either::Left(1);
        assert!(e.is_left());
    }

    #[test]
    fn test_is_right() {
        let e: Either<(), i32> = Either::Right(1);
        assert!(e.is_right());
    }

    #[test]
    fn test_is() {
        let e: Either<i32, ()> = Either::Left(1);
        assert!(e.is::<i32>());
    }

    #[test]
    fn structs() {
        #[derive(Debug, Clone)]
        struct A;

        let e: Either<A, ()> = Either::Left(A);
        assert!(e.is::<A>());
        assert!(e.is_left());
    }
}