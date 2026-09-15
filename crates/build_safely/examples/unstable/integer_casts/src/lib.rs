#![cfg_attr(unstable_integer_casts, feature(integer_casts))]
#![allow(unused)]

#[cfg(has_integer_casts)]
mod has {
    #[test]
    fn has() {
        let x: u32 = 0_usize.strict_cast();
    }
}

#[cfg(not(has_integer_casts))]
mod has_not {
    /// ```compile_fail
    /// let x: u32 = 0_usize.strict_cast();
    /// ```
    fn doctest() {}
}
