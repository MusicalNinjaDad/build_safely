#![cfg_attr(unstable_unsized_const_params, feature(unsized_const_params))]
#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]
#![cfg_attr(unstable_adt_const_params, expect(incomplete_features))]
#![allow(unused)]

#[cfg(has_unsized_const_params)]
mod has {
    use std::fmt::Display;

    struct Named<const NAME: &'static str>;

    impl<const NAME: &'static str> Display for Named<NAME> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{NAME}")
        }
    }

    #[test]
    fn name() {
        let bob = Named::<"Bob">;
        assert_eq!("Bob", format!("{bob}"));
    }
}

#[cfg(not(has_unsized_const_params))]
mod has_not {
    /// ```compile_fail
    /// use std::fmt::Display;
    ///
    /// struct Named<const NAME: &'static str>;
    ///
    /// impl<const NAME: &'static str> Display for Named<NAME> {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "{NAME}")
    ///     }
    /// }
    /// ```
    fn doctest() {}
}
