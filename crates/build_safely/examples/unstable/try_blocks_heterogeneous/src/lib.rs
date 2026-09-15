#![cfg_attr(unstable_try_blocks_heterogeneous, feature(try_blocks_heterogeneous))]
#![allow(unused)]

#[cfg(has_try_blocks_heterogeneous)]
mod has;

#[cfg(not(has_try_blocks_heterogeneous))]
mod has_not {
    /// ```compile_fail
    /// let err = try bikeshed Result<_, u16> {
    ///     let _ = Err(5_u8)?;
    ///     let _ = Err(6_u16)?;
    /// };
    /// assert_eq!(err, Err(5_u16));
    /// ```
    fn doctest() {}
}
