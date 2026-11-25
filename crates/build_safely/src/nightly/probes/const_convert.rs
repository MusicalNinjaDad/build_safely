pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
struct Foo(usize);
impl Foo {
    const fn new<N>(val: N) -> Self where
        usize: [const] From<N>,
    {
        Self(val.into())
    }
}
fn main() {}
"#;
