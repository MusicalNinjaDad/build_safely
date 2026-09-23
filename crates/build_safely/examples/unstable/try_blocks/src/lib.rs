#![cfg_attr(unstable_try_blocks, feature(try_blocks))]
#![allow(unused)]

#[cfg(has_try_blocks)]
mod has;

#[cfg(not(has_try_blocks))]
mod has_not {
    /// ```compile_fail
    /// let err: Result<(), u8> = try {
    ///     let _ = Err(5_u8)?;
    ///     let _ = Ok(6_u16)?;
    /// };
    /// assert_eq!(err, Err(5_u8));
    /// ```
    fn doctest() {}
}
