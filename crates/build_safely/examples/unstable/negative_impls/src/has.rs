#[test]
fn has() {
    struct Prisoner;
    impl !Send for Prisoner {}
}
