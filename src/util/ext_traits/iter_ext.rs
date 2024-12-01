use crate::util::gcd_lcm::GcdLcm;

pub trait IterExt<T>: Iterator<Item = T> {
    /// Find the lowest common multiple of all the elements in the iterator.
    ///
    /// Returns zero if there aren't any numbers in the list.
    fn lcm(self) -> T
    where
        T: GcdLcm;

    /// Find the greatest common divisor of all the elements in the iterator.
    ///
    /// Returns zero if there aren't any numbers in the list.
    fn gcd(self) -> T
    where
        T: GcdLcm;
}

impl<T, I> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn lcm(self) -> T
    where
        T: GcdLcm,
    {
        GcdLcm::lcm_list(self)
    }

    fn gcd(self) -> T
    where
        T: GcdLcm,
    {
        GcdLcm::gcd_list(self)
    }
}
