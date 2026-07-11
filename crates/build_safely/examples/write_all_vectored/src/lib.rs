#![cfg_attr(unstable_write_all_vectored, feature(write_all_vectored))]

#[cfg(test)]
#[cfg(has_write_all_vectored)]
mod tests {
    use std::io::{empty, Write, IoSlice};

    #[test]
    fn has() {
        let buf: [u8; _] = [0];
        let slice = IoSlice::new(&buf);
        empty().write_all_vectored(&mut [slice]).unwrap();
    }
}

#[cfg(test)]
#[cfg(not(has_write_all_vectored))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(0, 0);
    }
}
