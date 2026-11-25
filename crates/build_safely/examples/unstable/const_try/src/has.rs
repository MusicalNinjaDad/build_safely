const fn foo(maybe: Option<usize>) -> Option<usize> {
    let x = maybe?;
    Some(x + 1)
}

#[test]
fn has() {
    const FIVE: Option<usize> = foo(Some(4));
    assert_eq!(FIVE, Some(5));
}
