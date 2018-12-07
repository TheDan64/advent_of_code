use std::collections::HashSet;

#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let mut sum = 0;
    let num_strs = input.split('\n')
        .filter(|s| s.len() != 0);

    for num_str in num_strs {
        sum += num_str.parse::<i32>().unwrap();
    }

    sum
}

#[aoc(day1, part2, Chars)]
pub fn part2_chars(input: &str) -> i32 {
    let mut sum = 0;
    let mut frequencies = HashSet::new();
    let num_iter = input
        .split('\n')
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<i32>().unwrap())
        .cycle();

    frequencies.insert(0);

    for num in num_iter {
        sum += num;

        if frequencies.contains(&sum) {
            return sum;
        }

        frequencies.insert(sum);
    }

    unreachable!()
}
