const fn foo() -> bool {
    Some(5).is_some_and(oh_yes)
}

const fn oh_yes(_: usize) -> bool {
    true
}

#[test]
fn has() {
    const FIVE: bool = foo();
    const {
        assert!(FIVE);
    }
}
