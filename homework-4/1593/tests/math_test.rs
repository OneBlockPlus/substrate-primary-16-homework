use homework4::math::sum_u32;

#[test]
fn test_sum_u32_no_overflow() {
    let numbers = [1, 2, 3, 4];
    assert_eq!(sum_u32(&numbers), Some(10));
}

#[test]
fn test_sum_u32_with_overflow() {
    let numbers = [u32::MAX, 1];
    assert_eq!(sum_u32(&numbers), None);
}
