#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

#[cfg(test)]
#[cfg(all(has_never_type, has_try_trait_v2, has_try_trait_v2_residual))]
mod tests {
    use std::ops::{ControlFlow, Residual, Try};

    #[test]
    fn has_never_type() {
        type Bang = !;
    }

    #[test]
    fn has_try_trait_v2() {
        fn falls_through(x: u32) -> ControlFlow<!> {
            if x == 0 {
                ControlFlow::Break(continue)
            } else {
                ControlFlow::Continue(())
            }
        }
        let _ = falls_through(1);
    }

    #[test]
    fn has_try_trait_v2_residual() {
        fn branch(x: bool) -> Residual<()> {
            if x {
                Residual::Ok(())
            } else {
                Residual::Err(continue)
            }
        }
        let _ = branch(true);
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
