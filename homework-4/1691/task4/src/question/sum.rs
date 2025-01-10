pub fn sum(set: &[u32]) -> Option<u32> {
    if set.is_empty() {
        None
    }else {
        let mut answer:u32 = 0;
        for &i in set{
            answer = answer + i;
        }
        Some(answer)
    }
}