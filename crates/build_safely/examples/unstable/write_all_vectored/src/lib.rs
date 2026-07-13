#![cfg_attr(unstable_write_all_vectored, feature(write_all_vectored))]
#![allow(unused)]

#[cfg(has_write_all_vectored)]
mod has {
    use std::io::{IoSlice, Write, empty};

    #[test]
    fn has() {
        let buf: [u8; _] = [0];
        let slice = IoSlice::new(&buf);
        empty().write_all_vectored(&mut [slice]).unwrap();
    }
}

#[cfg(not(has_write_all_vectored))]
mod has_not {
    /// ```compile_fail
    /// use std::io::{IoSlice, Write, empty};
    /// let buf: [u8; _] = [0];
    /// let slice = IoSlice::new(&buf);
    /// empty().write_all_vectored(&mut [slice]).unwrap();
    /// ```
    #[test]
    fn has_not() {}
}
