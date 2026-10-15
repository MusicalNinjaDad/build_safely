#![cfg_attr(unstable_exact_div, feature(exact_div))]
#![allow(unused)]

#[cfg(has_exact_div)]
mod has {
    #[test]
    fn has() {
        let x: Option<usize> = 10_usize.div_exact(5);
        assert_eq!(x.unwrap(), 2);
        let y: Option<usize> = 10_usize.div_exact(3);
        assert!(y.is_none())
    }
}

#[cfg(not(has_exact_div))]
mod has_not {
    /// ```compile_fail
    /// let x: Option<usize> = 10_usize.div_exact(5);
    /// assert_eq!(x.unwrap(), 2);
    /// let y: Option<usize> = 10_usize.div_exact(3);
    /// assert!(y.is_none())
    /// ```
    fn doctest() {}
}
