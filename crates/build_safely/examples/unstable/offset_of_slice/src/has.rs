use std::mem::offset_of;

#[repr(C)]
struct Dst {
    head: usize,
    tail: [u8],
}

const TAIL: usize = offset_of!(Dst, tail);

#[test]
fn has() {
    const { assert!(TAIL == size_of::<usize>()) }
}
