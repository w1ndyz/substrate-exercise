pub fn sum(nums: &[u32]) -> Option<u32> {
    nums.iter().fold(Some(0), |some, &value| match some {
        Some(sum) => sum.checked_add(value),
        None => None,
    })
}
