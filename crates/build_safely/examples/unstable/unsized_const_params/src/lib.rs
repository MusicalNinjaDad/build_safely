#![cfg_attr(unstable_unsized_const_params, feature(unsized_const_params))]
#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]
#![expect(incomplete_features)]
#![allow(unused)]

use std::fmt::Display;

#[cfg(has_unsized_const_params)]
struct Named<const NAME: &'static str>;

#[cfg(has_unsized_const_params)]
impl<const NAME: &'static str> Display for Named<NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NAME}")
    }
}

#[cfg(test)]
#[cfg(has_unsized_const_params)]
mod has {
    use super::*;

    #[test]
    fn name() {
        let bob = Named::<"Bob">;
        assert_eq!("Bob", format!("{bob}"));
    }
}

#[cfg(test)]
#[cfg(not(has_unsized_const_params))]
mod has_not {
    #[test]
    fn assert_true() {
        assert!(true);
    }
}
