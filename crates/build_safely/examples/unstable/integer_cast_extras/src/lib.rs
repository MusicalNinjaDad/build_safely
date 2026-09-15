#![cfg_attr(unstable_integer_cast_extras, feature(integer_cast_extras))]
#![cfg_attr(unstable_integer_casts, feature(integer_casts))]
#![allow(unused)]

#[cfg(has_integer_cast_extras)]
mod has {
    #[test]
    fn has() {
        let x: u32 = 0_i32.strict_cast_unsigned();
    }
}

#[cfg(not(has_integer_cast_extras))]
mod has_not {
    /// ```compile_fail
    /// let x: u32 = 0_i32.strict_cast_unsigned();
    /// ```
    fn doctest() {}
}
