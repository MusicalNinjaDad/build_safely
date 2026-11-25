pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
const fn foo() -> bool {
    Some(5).is_some_and(oh_yes)
}

const fn oh_yes(_: usize) -> bool {
    true
}

fn main () {}
"#;
