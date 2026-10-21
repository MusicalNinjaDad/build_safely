#[test]
fn has() {
    use std::ffi::OsStr;
    let os_str = OsStr::new("foo");
    let (f, oo) = os_str.split_at(1);
    assert_eq!(f, "f");
    assert_eq!(oo, "oo");
}
