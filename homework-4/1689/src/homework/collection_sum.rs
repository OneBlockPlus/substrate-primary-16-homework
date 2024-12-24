pub fn get_sum(collection: &[u32]) -> Option<u32> {
    collection
        .iter()
        .try_fold(0u32, |acc, &x| acc.checked_add(x))
}
#[test]
fn test_get_sum() {
    struct TestCase {
        input: Vec<u32>,
        expected: Option<u32>,
    }
    let test_cases = vec![
        TestCase {
            input: vec![1, 2, 3, 4, 5],
            expected: Some(15),
        },
        TestCase {
            input: vec![],
            expected: Some(0),
        },
        TestCase {
            input: vec![u32::MAX, 1],
            expected: None,
        },
        TestCase {
            input: vec![42],
            expected: Some(42),
        },
        TestCase {
            input: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            expected: Some(55),
        },
        TestCase {
            input: vec![u32::MAX, u32::MAX],
            expected: None,
        },
    ];
    for test_case in test_cases {
        assert_eq!(get_sum(&test_case.input), test_case.expected);
    }
}
