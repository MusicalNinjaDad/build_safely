#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]
#![allow(unused)]

#[cfg(has_adt_const_params)]
mod has {
    use std::marker::ConstParamTy;

    #[derive(ConstParamTy, PartialEq, Eq)]
    struct Increment(i32);

    struct Counter<const INC: Increment>(i32);

    impl<const INC: Increment> Counter<INC> {
        fn inc(&mut self) {
            self.0 += INC.0;
        }
    }

    #[test]
    fn has() {
        let mut counter = Counter::<{ Increment(2) }>(0);
        counter.inc();
        assert_eq!(counter.0, 2)
    }
}

#[cfg(not(has_adt_const_params))]
mod has_not {
    /// ```compile_fail
    /// use std::marker::ConstParamTy;
    ///
    /// #[derive(ConstParamTy, PartialEq, Eq)]
    /// struct Increment(i32);
    ///
    /// struct Counter<const INC: Increment>(i32);
    ///
    /// impl<const INC: Increment> Counter<INC> {
    ///     fn inc(&mut self) {
    ///         self.0 += INC.0;
    ///     }
    /// }
    /// ```
    fn doctest() {}
}
