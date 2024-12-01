use crate::util::const_generics_iterators::CartesianProduct;

pub trait ArrayExt<T, const N: usize> {
    /// Return an iterator of the cartesian product of all the iterators
    /// contained within the array.
    fn cartesian_product(self) -> CartesianProduct<T, N>
    where
        T: Iterator + Clone;
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    fn cartesian_product(self) -> CartesianProduct<T, N>
    where
        T: Iterator + Clone,
    {
        CartesianProduct::new(self)
    }
}
