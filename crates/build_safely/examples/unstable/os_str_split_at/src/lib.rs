#![cfg_attr(unstable_os_str_split_at, feature(os_str_split_at))]
#![allow(unused)]

#[cfg(has_os_str_split_at)]
mod has;

#[cfg(not(has_os_str_split_at))]
mod has_not {
    /// ```compile_fail
    /// use std::ffi::OsStr;
    /// let os_str = OsStr::new("foo");
    /// let (f, oo) = os_str.split_at(1);
    /// assert_eq!(f, "f");
    /// assert_eq!(oo, "oo");
    /// ```
    fn doctest() {}
}
