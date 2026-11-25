#![cfg_attr(unstable_ptr_metadata, feature(ptr_metadata))]
#![allow(unused)]

#[cfg(has_ptr_metadata)]
mod has;

#[cfg(not(has_ptr_metadata))]
mod has_not {
    /// ```compile_fail
    /// use core::ptr::{NonNull, Pointee};
    ///
    ///     let s = [1,2].as_slice();
    ///     let wide = NonNull::from_ref(s);
    ///     let (thin, meta) = wide.to_raw_parts();
    ///     assert_eq!(meta, 2);
    /// ```
    fn doctest() {}
}
