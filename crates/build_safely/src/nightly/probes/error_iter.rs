pub const AVAILABLE: &str = r#"
use std::{error::Error, io};

fn main() {
    let e = &io::Error::other("oops") as &dyn Error;
    let _s = e.sources();
}
"#;
