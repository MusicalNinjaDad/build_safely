pub const AVAILABLE: &str = r#"
use std::mem::offset_of;

struct Dst {
    head: usize,
    tail: [u8],
}

const TAIL: usize = offset_of!(Dst, tail);

fn main () {}
"#;
