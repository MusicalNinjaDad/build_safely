pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
const fn same(lhs: u8, rhs: u8) -> bool {
    lhs.eq(&rhs)
}
fn main() {}
"#;
