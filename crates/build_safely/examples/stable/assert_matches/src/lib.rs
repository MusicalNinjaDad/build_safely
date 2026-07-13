#![cfg_attr(unstable_assert_matches, feature(assert_matches))]
#![allow(unused)]

#[cfg(has_assert_matches)]
mod has {
    #[cfg(assert_matches_location = "root")]
    use std::assert_matches;

    #[cfg(assert_matches_location = "module")]
    use std::assert_matches::assert_matches;

    #[test]
    fn has() {
        assert_matches!(Some(5), Some(n) if n == 5);
    }
}

#[cfg(not(has_assert_matches))]
mod has_not {
    /// ```compile_fail
    /// use std::assert_matches;
    /// ```
    fn doctest() {}
}
