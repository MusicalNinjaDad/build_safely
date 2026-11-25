pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
use std::marker::Destruct;

struct Foo;

impl Foo {
    const fn wibble<N>(val: N) -> N where
        N: [const] Destruct,
    {
        val
    }
}
fn main() {}
"#;
