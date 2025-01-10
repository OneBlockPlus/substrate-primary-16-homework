// 实现一个函数，为 u32 类型的整数集合求和，参数类型为 &[u32]，返回类型为 Option，溢出时返回 None；

/// 计算 u32 整数集合的和，如果发生溢出则返回 None。
///
/// # Examples
///
/// ```
/// use sum_u32;
///
/// assert_eq!(sum_u32(&[1, 2, 3]), Some(6));
/// assert_eq!(sum_u32(&[]), Some(0));
/// assert_eq!(sum_u32(&[u32::MAX, 1]), None);
/// ```
///
pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers
        .iter()
        .try_fold(0u32, |acc: u32, &x: &u32| acc.checked_add(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        assert_eq!(sum_u32(&[1, 2, 3, 4, 5]), Some(15));
        assert_eq!(sum_u32(&[u32::MAX, 1]), None);
    }

    #[test]
    fn test_sum_u32_empty() {
        assert_eq!(sum_u32(&[]), Some(0));
    }

    #[test]
    fn test_sum_u32_normal() {
        assert_eq!(sum_u32(&[1, 2, 3]), Some(6));
    }

    #[test]
    fn test_sum_u32_overflow() {
        assert_eq!(sum_u32(&[u32::MAX, 1]), None);
    }

    #[test]
    fn test_sum_u32_large_numbers() {
        assert_eq!(sum_u32(&[1000, 2000, 3000]), Some(6000));
    }

    #[test]
    fn test_sum_u32_zero() {
        assert_eq!(sum_u32(&[0, 0, 0]), Some(0));
    }
}
