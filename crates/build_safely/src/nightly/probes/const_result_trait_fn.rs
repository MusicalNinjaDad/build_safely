pub const AVAILABLE: &str = r#"
#![allow(clippy::duplicated_attributes)]
#![allow(stable_features)]        
#![feature(const_trait_impl)]
const fn foo() -> bool {
    Ok::<_,()>(5).is_ok_and(oh_yes)
}

const fn oh_yes(_: usize) -> bool {
    true
}

fn main () {}
"#;
