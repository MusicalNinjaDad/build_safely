#![cfg_attr(unstable_can_vector, feature(can_vector))]
#![allow(unused)]

#[cfg(has_can_vector)]
mod has {
    use std::io::Read;

    #[test]
    fn has() {
        let _ = std::io::empty().is_read_vectored();
    }
}

#[cfg(not(has_can_vector))]
mod has_not {
    /// ```compile_fail
    /// use std::io::Read;
    /// let _ = std::io::empty().is_read_vectored();
    /// ```
    fn doctest() {}
}
