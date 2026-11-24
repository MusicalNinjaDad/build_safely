const THEN_5: Option<u8> = true.then_some(5);
const THEN_NONE: Option<u8> = false.then_some(5);
const THEN_SOME_9: Option<u8> = true.then(nine);
const THEN_SOME_NONE: Option<u8> = false.then(nine);
const OK_OR_5: Result<(), u8> = true.ok_or(5);
const OK_OR_ERR_5: Result<(), u8> = false.ok_or(5);
const OK_OR_ELSE_9: Result<(), u8> = true.ok_or_else(nine);
const OK_OR_ELSE_ERR_9: Result<(), u8> = false.ok_or_else(nine);

const fn nine() -> u8 {
    9
}

#[test]
fn has() {
    assert_eq!(THEN_5, Some(5));
    assert_eq!(THEN_NONE, None);
    assert_eq!(THEN_SOME_9, Some(9));
    assert_eq!(THEN_SOME_NONE, None);
    assert_eq!(OK_OR_5, Ok(()));
    assert_eq!(OK_OR_ERR_5, Err(5));
    assert_eq!(OK_OR_ELSE_9, Ok(()));
    assert_eq!(OK_OR_ELSE_ERR_9, Err(9));
}
