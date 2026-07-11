#![cfg_attr(unstable_unsized_const_params, feature(unsized_const_params))]
#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]

#[cfg(test)]
#[cfg(has_unsized_const_params)]
mod tests {
    #[test]
    fn has() {
        struct Foo<const N: &'static str>;
        let _: Foo<"test">;
    }
}

#[cfg(test)]
#[cfg(not(has_unsized_const_params))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
