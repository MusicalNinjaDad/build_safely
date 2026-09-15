#![cfg_attr(unstable_iter_next_chunk, feature(iter_next_chunk))]
#![allow(unused)]

#[cfg(has_iter_next_chunk)]
mod has {
    #[test]
    fn has() {
        let x = [0, 1, 2, 3, 4];
        let first: Result<[_; _], std::array::IntoIter<_, _>> = x.iter().next_chunk::<2>();
        let last: Result<[_; _], std::array::IntoIter<_, _>> = x.iter().next_chunk_back::<2>();
        assert_eq!(first.unwrap(), [&0, &1]);
        assert_eq!(last.unwrap(), [&3, &4]);
    }
}

#[cfg(not(has_iter_next_chunk))]
mod has_not {
    /// ```compile_fail
    /// let x = [0,1,2,3,4];
    /// let first: Result<[_;_],std::array::IntoIter<_,_>> = x.iter().next_chunk::<2>();
    /// let last: Result<[_;_],std::array::IntoIter<_,_>> = x.iter().next_chunk_back::<2>();
    /// assert_eq!(first.unwrap(), [&0,&1]);
    /// assert_eq!(last.unwrap(), [&3,&4]);
    /// ```
    fn doctest() {}
}
