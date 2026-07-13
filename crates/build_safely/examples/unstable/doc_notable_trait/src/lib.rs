#![cfg_attr(unstable_doc_notable_trait, feature(doc_notable_trait))]
#![allow(unused)]

#[cfg(has_doc_notable_trait)]
mod has {
    #[test]
    fn has() {
        #[doc(notable_trait)]
        trait Foo {}
    }
}

#[cfg(not(has_doc_notable_trait))]
mod has_not {
    /// ```compile_fail
    /// #[doc(notable_trait)]
    /// trait Foo {}
    /// ```
    #[test]
    fn has_not() {}
}
