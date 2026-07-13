#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]
#![allow(unused)]

#[cfg(all(has_never_type, has_try_trait_v2, has_try_trait_v2_residual))]
mod has {
    use std::ops::{ControlFlow, Try};

    #[test]
    fn has() {
        // Verify all three features work together
        // Never type
        fn diverges() -> ! {
            panic!()
        }
        let _: fn() -> ! = diverges;

        // Try trait v2
        let _: ControlFlow<()> = ControlFlow::Continue(());

        // Try trait v2 residual
        use std::ops::Residual;
        fn _test<T: Residual<()>>(_: T) {}
    }
}

#[cfg(not(all(has_never_type, has_try_trait_v2, has_try_trait_v2_residual)))]
mod has_not {
    /// ```compile_fail
    /// fn diverges() -> ! {
    ///     panic!()
    /// }
    /// ```
    #[test]
    fn has_not() {}
}
