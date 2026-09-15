use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Frame(usize);

const impl Add<Frame> for Frame {
    type Output = Self;

    fn add(self, rhs: Frame) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

const impl Sub<Frame> for Frame {
    type Output = Self;

    fn sub(self, rhs: Frame) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

const ONE: Frame = Frame(1);
const TWO: Frame = ONE + ONE;

#[test]
fn has() {
    assert_eq!(TWO, Frame(2))
}
