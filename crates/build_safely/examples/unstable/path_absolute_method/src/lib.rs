#![cfg_attr(unstable_path_absolute_method, feature(path_absolute_method))]
#![allow(unused)]

#[cfg(has_path_absolute_method)]
mod has {
    #[test]
    fn has() {
        use std::{io, path::PathBuf};
        let p = PathBuf::from("some.file");
        let abs: Result<PathBuf, io::Error> = p.absolute();
        assert!(abs.unwrap().ends_with("some.file"));
    }
}

#[cfg(not(has_path_absolute_method))]
mod has_not {
    /// ```compile_fail
    /// use std::{io, path::PathBuf};
    /// let p = PathBuf::from("some.file");
    /// let abs: Result<PathBuf, io::Error> = p.absolute();
    /// assert!(abs.unwrap().ends_with("some.file"));
    /// ```
    fn doctest() {}
}
