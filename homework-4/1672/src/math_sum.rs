pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers
        .iter()
        .try_fold(0u32, |acc: u32, &x: &u32| acc.checked_add(x))
}