use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(nums: &[u32]) -> u32 {
    let mut map = HashSet::with_capacity(nums.len());

    // Optimized solution: O(n) worst case
    for num in nums.iter().copied() {
        if map.get(&(2020 - num)).is_some() {
            return num * (2020 - num);
        }

        map.insert(num);
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(nums: &[u32]) -> u32 {
    let mut map = HashSet::with_capacity(nums.len());

    // Optimized solution: O(n^2 - n) worst case
    for (i, num) in nums.iter().copied().enumerate() {
        for num2 in &nums[i + 1..] {
            if map.get(&(2020 - num - num2)).is_some() {
                return num * num2 * (2020 - num - num2);
            }
        }

        map.insert(num);
    }

    unreachable!()
}
