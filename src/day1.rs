#[aoc(day1, part1)]
pub fn part1_chars(input: &str) -> u32 {
    let num_strs: Vec<_> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // Optimized solution
    for (i, num) in num_strs.iter().enumerate() {
        for num2 in &num_strs[i + 1..] {
            if num + num2 == 2020 {
                return num * num2;
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2, Chars)]
pub fn part2_chars(input: &str) -> u32 {
    let num_strs: Vec<_> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // More performant than before, but still naive solution
    for (i, num) in num_strs.iter().enumerate() {
        for (j, num2) in num_strs[i + 1..].iter().enumerate() {
            for num3 in &num_strs[i + j + 2..] {
                if num + num2 + num3 == 2020 {
                    return num * num2 * num3;
                }
            }
        }
    }

    unreachable!()
}
