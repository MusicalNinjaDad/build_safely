#![cfg_attr(unstable_doc_notable_trait, feature(doc_notable_trait))]

#[cfg(test)]
#[cfg(has_doc_notable_trait)]
mod tests {
    #[test]
    fn has() {
        #[doc(notable_trait)]
        trait Foo {}
    }
}

#[cfg(test)]
#[cfg(not(has_doc_notable_trait))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
