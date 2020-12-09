use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split('\n').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(nums: &[u64]) -> u64 {
    const PREAMBLE: usize = 25;

    let mut set: HashSet<_> = nums.iter().copied().take(PREAMBLE).collect();

    'outer: for (i, num) in nums.iter().copied().skip(PREAMBLE).enumerate() {
        for item in nums[i..i + PREAMBLE].iter().copied() {
            if set.get(&(num - item)).is_some() {
                set.remove(&nums[i]);
                set.insert(num);
                continue 'outer;
            }
        }

        return nums[i + PREAMBLE];
    }

    unreachable!()
}

#[aoc(day9, part2)]
pub fn part2(nums: &[u64]) -> u64 {
    let p1 = part1(nums);

    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i..=j].iter().copied().sum::<u64>() == p1 {
                return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap();
            }
        }
    }

    unreachable!()
}
