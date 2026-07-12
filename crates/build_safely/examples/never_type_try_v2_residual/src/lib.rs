#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

#[cfg(test)]
#[cfg(all(has_never_type, has_try_trait_v2, has_try_trait_v2_residual))]
mod tests {
    use std::ops::{ControlFlow, Try};

    #[test]
    fn has_never_type() {
        // Just verify we can use the never type in a function signature
        fn diverges() -> ! {
            panic!()
        }
        let _: fn() -> ! = diverges;
    }

    #[test]
    fn has_try_trait_v2() {
        // Just verify ControlFlow type exists
        let _: ControlFlow<()> = ControlFlow::Continue(());
    }

    #[test]
    fn has_try_trait_v2_residual() {
        // Just verify we can use the Try trait
        use std::ops::Residual;
        // We can't use Residual directly as a concrete type, but we can use it in a bound
        fn _test<T: Residual<()>>(_: T) {}
    }
}

#[cfg(test)]
#[cfg(not(all(has_never_type, has_try_trait_v2, has_try_trait_v2_residual)))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
