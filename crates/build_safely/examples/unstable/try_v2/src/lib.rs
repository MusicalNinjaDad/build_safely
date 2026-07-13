#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]
#![allow(unused)]

#[cfg(all(has_try_trait_v2, has_try_trait_v2_residual))]
mod has {
    use std::{
        num::NonZeroI32,
        ops::{ControlFlow, FromResidual, Residual, Try},
    };

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct ResultCode(pub i32);
    pub struct ResultCodeResidual(NonZeroI32);

    impl ResultCode {
        const SUCCESS: Self = ResultCode(0);
    }

    impl Try for ResultCode {
        type Output = ();
        type Residual = ResultCodeResidual;

        fn branch(self) -> ControlFlow<Self::Residual> {
            match NonZeroI32::new(self.0) {
                Some(r) => ControlFlow::Break(ResultCodeResidual(r)),
                None => ControlFlow::Continue(()),
            }
        }

        fn from_output(_: ()) -> Self {
            ResultCode::SUCCESS
        }
    }

    impl FromResidual for ResultCode {
        fn from_residual(r: ResultCodeResidual) -> Self {
            ResultCode(r.0.into())
        }
    }

    impl Residual<()> for ResultCodeResidual {
        type TryType = ResultCode;
    }

    fn check_code(n: i32) -> ResultCode {
        ResultCode(n)?;
        assert_eq!(n, 0);
        ResultCode::SUCCESS
    }

    #[test]
    fn has() {
        assert_eq!(check_code(0), ResultCode::SUCCESS);
        assert_eq!(check_code(2), ResultCode(2));
    }
}

#[cfg(not(all(has_try_trait_v2, has_try_trait_v2_residual)))]
mod has_not {
    /// ```compile_fail
    /// use std::{num::NonZeroI32, ops::{ControlFlow, FromResidual, Residual, Try}};
    ///
    /// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    /// pub struct ResultCode(pub i32);
    /// pub struct ResultCodeResidual(NonZeroI32);
    ///
    /// impl ResultCode {
    ///     const SUCCESS: Self = ResultCode(0);
    /// }
    ///
    /// impl Try for ResultCode {
    ///     type Output = ();
    ///     type Residual = ResultCodeResidual;
    ///
    ///     fn branch(self) -> ControlFlow<Self::Residual> {
    ///         match NonZeroI32::new(self.0) {
    ///             Some(r) => ControlFlow::Break(ResultCodeResidual(r)),
    ///             None => ControlFlow::Continue(()),
    ///         }
    ///     }
    ///
    ///     fn from_output(_: ()) -> Self {
    ///         ResultCode::SUCCESS
    ///     }
    /// }
    ///
    /// impl FromResidual for ResultCode {
    ///     fn from_residual(r: ResultCodeResidual) -> Self {
    ///         ResultCode(r.0.into())
    ///     }
    /// }
    ///
    /// impl Residual<()> for ResultCodeResidual {
    ///     type TryType = ResultCode;
    /// }
    ///
    /// fn check_code(n: i32) -> ResultCode {
    ///     ResultCode(n)?;
    ///     assert_eq!(n, 0);
    ///     ResultCode::SUCCESS
    /// }
    /// ```
    fn doctest() {}
}
