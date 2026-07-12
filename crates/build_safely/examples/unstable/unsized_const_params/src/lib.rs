#![cfg_attr(unstable_unsized_const_params, feature(unsized_const_params))]
#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]
#![expect(incomplete_features)]

#[cfg(test)]
#[cfg(has_unsized_const_params)]
mod has {
    #[test]
    fn has() {
        struct Foo<const N: &'static str>;
        let _: Foo<"test">;
    }
}

#[cfg(test)]
#[cfg(not(has_unsized_const_params))]
mod has_not {
    #[test]
    fn has_not() {
        assert!(true);
    }
}
