#![cfg_attr(unstable_const_ops, feature(const_ops))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_ops)]
mod has;

#[cfg(not(has_const_ops))]
mod has_not {
    /// ```compile_fail
    /// use std::ops::{Add, Sub};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    /// struct Frame(usize);
    ///
    /// const impl Add<Frame> for Frame {
    ///      type Output = Self;
    ///
    /// fn add(self, rhs: Frame) -> Self::Output {
    ///          Self(self.0 + rhs.0)
    ///      }
    ///  }
    ///
    ///  const impl Sub<Frame> for Frame {
    ///      type Output = Self;
    ///
    ///  fn sub(self, rhs: Frame) -> Self::Output {
    ///          Self(self.0 - rhs.0)
    ///      }
    ///  }
    ///
    ///  const ONE: Frame = Frame(1);
    ///  const TWO: Frame = ONE + ONE;
    ///
    ///   #[test]
    ///  fn has() {
    ///      assert_eq!(TWO, Frame(2))
    ///  }
    ///  ```
    fn doctest() {}
}
