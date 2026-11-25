#![cfg_attr(unstable_offset_of_slice, feature(offset_of_slice))]
#![allow(unused)]

#[cfg(has_offset_of_slice)]
mod has;

#[cfg(not(has_offset_of_slice))]
mod has_not {
    /// ```compile_fail
    /// use std::mem::offset_of;
    ///
    /// #[repr(C)]
    /// struct Dst {
    ///     head: usize,
    ///     tail: [u8],
    /// }
    ///
    /// const TAIL: usize = offset_of!(Dst, tail);
    ///
    ///     const { assert!(TAIL == size_of::<usize>()) }
    /// ```
    fn doctest() {}
}
