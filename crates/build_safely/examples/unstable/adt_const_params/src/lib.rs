#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]
#![allow(unused)]

#[cfg(has_adt_const_params)]
use std::marker::ConstParamTy;

#[cfg(has_adt_const_params)]
#[derive(ConstParamTy, PartialEq, Eq)]
struct Increment(i32);

#[cfg(has_adt_const_params)]
struct Counter<const INC: Increment>(i32);

#[cfg(has_adt_const_params)]
impl<const INC: Increment> Counter<INC> {
    fn inc(&mut self) {
        self.0 += INC.0;
    }
}

#[cfg(test)]
#[cfg(has_adt_const_params)]
mod has {
    use super::*;

    #[test]
    fn has() {
        let mut counter = Counter::<{ Increment(2) }>(0);
        counter.inc();
        assert_eq!(counter.0, 2)
    }
}

#[cfg(test)]
#[cfg(not(has_adt_const_params))]
mod has_not {
    #[test]
    fn has_not() {
        assert!(true);
    }
}
