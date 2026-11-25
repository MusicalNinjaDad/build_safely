#![cfg_attr(unstable_structural_match, feature(structural_match))]
#![allow(unused)]

#[cfg(has_structural_match)]
mod has;

#[cfg(not(has_structural_match))]
mod has_not {
    /// ```compile_fail
    /// #![allow(clippy::assertions_on_constants)]
    /// use core::marker::StructuralPartialEq;
    ///
    /// struct Number(usize);
    ///
    /// impl PartialEq for Number {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.0 == other.0
    ///     }
    /// }
    ///
    /// impl StructuralPartialEq for Number {}
    ///
    ///     let five = Number(5);
    ///     match five {
    ///         Number(5) => assert!(true),
    ///         _ => assert!(false),
    ///     }
    /// ```
    fn doctest() {}
}
