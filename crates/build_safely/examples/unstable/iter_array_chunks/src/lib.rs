#![cfg_attr(unstable_iter_array_chunks, feature(iter_array_chunks))]
#![allow(unused)]

#[cfg(has_iter_array_chunks)]
mod has {
    #[test]
    fn has() {
        let x = [0, 1, 2, 3, 4];
        let chunked = x.iter().array_chunks::<2>();
        let pairs: Vec<_> = chunked.clone().collect();
        let last: Vec<_> = chunked.clone().into_remainder().collect();
        assert_eq!(pairs, vec![[&0, &1], [&2, &3]]);
        assert_eq!(last, vec![&4]);
    }
}

#[cfg(not(has_iter_array_chunks))]
mod has_not {
    /// ```compile_fail
    /// let x = [0, 1, 2, 3, 4];
    /// let chunked = x.iter().array_chunks::<2>();
    /// let pairs: Vec<_> = chunked.clone().collect();
    /// let last: Vec<_> = chunked.clone().into_remainder().collect();
    /// assert_eq!(pairs, vec![[&0, &1], [&2, &3]]);
    /// assert_eq!(last, vec![&4]);
    /// ```
    fn doctest() {}
}
