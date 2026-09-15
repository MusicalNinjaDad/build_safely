#[test]
fn has() {
    let err = try bikeshed Result<_, u16> {
        let _ = Err(5_u8)?;
        let _ = Err(6_u16)?;
    };
    assert_eq!(err, Err(5_u16));
}
