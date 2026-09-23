#[test]
fn has() {
    let err: Result<(), u8> = try {
        let _ = Err(5_u8)?;
        let _ = Ok(6_u16)?;
    };
    assert_eq!(err, Err(5_u8));
}
