#![allow(clippy::assertions_on_constants)]
trait GoodError = core::error::Error + Send + Sync + 'static;

#[test]
fn has() {
    assert!(true)
}
