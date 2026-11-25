const fn comparisons(lhs: u8, rhs: u8) -> bool {
    lhs.ge(&rhs);
    lhs.gt(&rhs);
    lhs.le(&rhs);
    lhs.lt(&rhs);
    lhs.ne(&rhs);
    lhs.eq(&rhs)
}

#[test]
fn has() {
    const TRUE: bool = comparisons(5, 5);
    const {
        assert!(TRUE);
    }
}
