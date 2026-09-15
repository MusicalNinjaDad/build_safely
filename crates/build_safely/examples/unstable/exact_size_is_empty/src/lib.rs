#![cfg_attr(unstable_exact_size_is_empty, feature(exact_size_is_empty))]
#![allow(unused)]

#[cfg(has_exact_size_is_empty)]
mod has {
    #[test]
    fn has() {
        struct Empty;

        impl Iterator for Empty {
            type Item = ();
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

        impl ExactSizeIterator for Empty {
            fn len(&self) -> usize {
                0
            }
            fn is_empty(&self) -> bool {
                true
            }
        }

        let x = Empty;
        assert!(x.is_empty());
    }
}

#[cfg(not(has_exact_size_is_empty))]
mod has_not {
    /// ```compile_fail
    /// struct Empty;
    ///
    /// impl Iterator for Empty {
    ///     type Item = ();
    ///     fn next(&mut self) -> Option<Self::Item> {
    ///         None
    ///     }
    /// }
    ///
    /// impl ExactSizeIterator for Empty {
    ///     fn len(&self) -> usize {
    ///         0
    ///     }
    ///     fn is_empty(&self) -> bool {
    ///         true
    ///     }
    /// }
    ///
    /// let x = Empty;
    /// assert!(x.is_empty());
    /// ```
    fn doctest() {}
}
