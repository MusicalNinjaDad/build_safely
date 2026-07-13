#![cfg_attr(unstable_bool_to_result, feature(bool_to_result))]
#![allow(unused)]

#[cfg(has_bool_to_result)]
mod has {
    #[test]
    fn has() {
        let _ = true.ok_or(());
        let _ = false.ok_or_else(|| ());
    }
}

#[cfg(not(has_bool_to_result))]
mod has_not {
    /// ```compile_fail
    /// let _ = true.ok_or(());
    /// ```
    #[test]
    fn has_not() {}
}
