pub fn sum_u32(slice: &[u32]) -> Option<u32> {
    slice.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}
