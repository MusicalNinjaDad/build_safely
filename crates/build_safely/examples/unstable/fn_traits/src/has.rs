use core::cmp::min;

#[test]
fn has() {
    let nums = (1, 2);
    let small = min.call(nums);
    assert_eq!(small, 1);
}
