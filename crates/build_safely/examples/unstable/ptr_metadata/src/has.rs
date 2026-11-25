use core::ptr::{NonNull, Pointee};

#[test]
fn has() {
    let s = [1, 2].as_slice();
    let wide = NonNull::from_ref(s);
    let (thin, meta) = wide.to_raw_parts();
    assert_eq!(meta, 2);
}
