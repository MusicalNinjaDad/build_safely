#![cfg_attr(unstable_strip_circumfix, feature(strip_circumfix))]
#![allow(unused)]

#[cfg(has_strip_circumfix)]
mod has {
    #[test]
    fn has() {
        let s = "foo";
        let _ = s.strip_circumfix("f", "o");
    }
}

#[cfg(not(has_strip_circumfix))]
mod has_not {
    /// ```compile_fail
    /// let s = "foo";
    /// let _ = s.strip_circumfix("f", "o");
    /// ```
    #[test]
    fn has_not() {}
}
