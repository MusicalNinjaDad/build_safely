#![cfg_attr(unstable_bool_to_result, feature(bool_to_result))]
#![allow(unused)]
#![allow(clippy::unnecessary_lazy_evaluations)]

#[cfg(has_bool_to_result)]
mod has {
    #[test]
    fn has() {
        assert_eq!(true.ok_or(0), Ok(()));
        assert_eq!(false.ok_or_else(|| 1+1), Err(2));
    }
}

#[cfg(not(has_bool_to_result))]
mod has_not {
    /// ```compile_fail
    /// let _ = true.ok_or(());
    /// ```
    fn doctest() {}
}
