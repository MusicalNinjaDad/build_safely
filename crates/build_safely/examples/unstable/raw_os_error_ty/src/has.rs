use std::io::{self, RawOsError};

#[test]
fn has() {
    let err = io::Error::other("custom");
    let raw: Option<RawOsError> = err.raw_os_error();
    assert_eq!(raw, None);
}
