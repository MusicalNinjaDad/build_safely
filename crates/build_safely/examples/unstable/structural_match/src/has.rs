#![allow(clippy::assertions_on_constants)]
use core::marker::StructuralPartialEq;

struct Number(usize);

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl StructuralPartialEq for Number {}

#[test]
fn has() {
    let five = Number(5);
    match five {
        Number(5) => assert!(true),
        _ => assert!(false),
    }
}
