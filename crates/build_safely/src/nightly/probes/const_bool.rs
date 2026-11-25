pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
fn main() {
    const THEN_5: Option<u8> = true.then_some(5);
}
"#;
