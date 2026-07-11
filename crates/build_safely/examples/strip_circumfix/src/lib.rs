#![cfg_attr(unstable_strip_circumfix, feature(strip_circumfix))]

#[cfg(test)]
#[cfg(has_strip_circumfix)]
mod tests {
    #[test]
    fn has() {
        let s = "foo";
        let _ = s.strip_circumfix("f", "o");
    }
}

#[cfg(test)]
#[cfg(not(has_strip_circumfix))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!("foo", "foo");
    }
}
