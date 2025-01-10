/// 为 u32 类型的整数集合求和，溢出时返回 None
pub fn sum_with_overflow_check(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}

/// 编写单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_with_overflow_check() {
        let numbers = [1, 2, 3, u32::MAX];
        // 溢出
        assert_eq!(sum_with_overflow_check(&numbers), None);
        // 不溢出
        assert_eq!(sum_with_overflow_check(&[1, 2, 3]), Some(6));
    }
}