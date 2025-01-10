fn sum_u32(slice: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in slice {
        if let Some(s) = sum.checked_add(num) {
            sum = s;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result1 = sum_u32(&numbers);
    println!("{:?}", result1);
    let numbers1 = vec![u32::MAX,1];
    let result2 = sum_u32(&numbers1);
    println!("{:?}", result2);
}