const fn foo() -> bool {
    Ok::<_, ()>(5).is_ok_and(oh_yes)
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
