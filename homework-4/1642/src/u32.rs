fn sum_u32(slice: &[u32]) -> Option<u32> {
    slice.iter().fold(None, |acc, &item| {
        match acc {
            None => None, // 如果累加器是 None，表示溢出已经发生
            Some(acc) => item.checked_add(acc), // 尝试添加当前项，如果溢出则返回 None
        }
    })
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = sum_u32(&numbers);
    println!("The sum is: {:?}", result); // 正常情况输出总和

    let large_numbers = vec![u32::MAX; 3]; // 一个包含多个 u32::MAX 的向量
    let result = sum_u32(&large_numbers);
    println!("The sum is: {:?}", result); // 溢出情况输出 None
}