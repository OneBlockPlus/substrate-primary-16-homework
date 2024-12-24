//实现一个函数，为 u32 类型的整数集合求和，参数类型为 &[u32]，返回类型为 Option，溢出时返回 None；
pub  fn array_sum_u32(numbers: &Vec<u32>) -> Option<u32> {

    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}
/// 编写单元测试
#[cfg(test)]
mod tests_sum {
    use super::*;

    #[test]
    fn test_array_sum_u32() {
        let numbers = vec![1, 2, 3, u32::MAX];
        // 溢出
        assert_eq!(array_sum_u32(&numbers), None);
        // 不溢出
        assert_eq!(array_sum_u32(&vec![1, 2, 3]), Some(6));
    }
}