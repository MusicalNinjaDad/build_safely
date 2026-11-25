#![cfg_attr(unstable_raw_os_error_ty, feature(raw_os_error_ty))]
#![allow(unused)]

#[cfg(has_raw_os_error_ty)]
mod has;

#[cfg(not(has_raw_os_error_ty))]
mod has_not {
    /// ```compile_fail
    /// use std::io::{self, RawOsError};
    ///
    ///     let err = io::Error::other("custom");
    ///     let raw: Option<RawOsError> = err.raw_os_error();
    ///     assert_eq!(raw, None);
    /// ```
    fn doctest() {}
}
