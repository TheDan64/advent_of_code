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
    // Optimized solution
    for (i, num) in nums.iter().enumerate() {
        for num2 in &nums[i + 1..] {
            if num + num2 == 2020 {
                return num * num2;
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(nums: &[u32]) -> u32 {
    // Optimized solution
    for (i, num) in nums.iter().enumerate() {
        for (j, num2) in nums[i + 1..].iter().enumerate() {
            for num3 in &nums[i + j + 2..] {
                if num + num2 + num3 == 2020 {
                    return num * num2 * num3;
                }
            }
        }
    }

    unreachable!()
}
