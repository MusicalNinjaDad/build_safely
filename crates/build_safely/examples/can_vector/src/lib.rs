#![cfg_attr(unstable_can_vector, feature(can_vector))]

#[cfg(test)]
#[cfg(has_can_vector)]
mod tests {
    use std::io::Read;

    #[test]
    fn has() {
        let _ = std::io::empty().is_read_vectored();
    }
}

#[cfg(test)]
#[cfg(not(has_can_vector))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
