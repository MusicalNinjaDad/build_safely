#![cfg_attr(unstable_iterator_try_collect, feature(iterator_try_collect))]
#![allow(unused)]

#[cfg(has_iterator_try_collect)]
mod has {
    #[test]
    fn has() {
        let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
    }
}

#[cfg(not(has_iterator_try_collect))]
mod has_not {
    /// ```compile_fail
    /// let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
    /// ```
    #[test]
    fn has_not() {}
}
