#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]

#[cfg(test)]
#[cfg(has_adt_const_params)]
mod has {
    #[test]
    fn has() {
        struct Foo<const N: usize>;
        let _: Foo<5>;
    }
}

#[cfg(test)]
#[cfg(not(has_adt_const_params))]
mod has_not {
    #[test]
    fn has_not() {
        assert_eq!(5, 5);
    }
}
