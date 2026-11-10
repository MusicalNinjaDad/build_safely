#![cfg_attr(unstable_never_type, feature(never_type))]
#![allow(clippy::all)]
#![allow(unused)]

#[cfg(has_never_type)]
mod has {

    #[test]
    fn has() {
        let nothing: Option<!> = None;
        let n = match nothing {
            None => 0,
        };
        assert_eq!(n, 0);
    }
}

#[cfg(not(has_never_type))]
mod has_not {
    /// ```compile_fail
    /// let nothing: Option<!> = None;
    /// let n = match nothing {
    ///     None => 0,
    /// };
    /// ```
    fn doctest() {}
}
