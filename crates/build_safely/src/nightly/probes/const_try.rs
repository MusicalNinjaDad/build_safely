pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
const fn foo(maybe: Option<usize>) -> Option<usize> {
    let x = maybe?;
    Some(x + 1)
}

fn main () {}
"#;
