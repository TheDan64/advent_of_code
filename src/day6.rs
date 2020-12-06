use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let lines = input.split('\n');
    let mut sum = 0;
    let mut set = HashSet::new();

    for line in lines {
        if line.is_empty() {
            sum += set.len();
            set.clear();
            continue;
        }

        for ch in line.chars() {
            set.insert(ch);
        }
    }

    if !set.is_empty() {
        sum += set.len();
    }

    sum
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.split('\n');
    let mut sum = 0;
    let mut set = HashSet::new();
    let mut group_start = true;

    for line in lines {
        if line.is_empty() {
            sum += set.len();
            set.clear();
            group_start = true;
            continue;
        }

        let mut local_set = HashSet::new();

        for ch in line.chars() {
            if group_start {
                set.insert(ch);
            } else {
                local_set.insert(ch);
            }
        }

        if !group_start {
            set = set.intersection(&local_set).copied().collect();
        }

        group_start = false;
    }

    if !set.is_empty() {
        sum += set.len();
    }

    sum
}
