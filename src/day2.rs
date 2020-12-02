use std::collections::HashMap;

#[derive(Debug)]
pub struct Counter(HashMap<char, usize>);

impl Counter {
    fn new(s: &str) -> Self {
        let mut counter = Counter(HashMap::new());

        for ch in s.chars() {
            counter.add_char(ch);
        }

        counter
    }

    fn add_char(&mut self, ch: char) {
        let entry = self.0.entry(ch).or_insert(0);

        *entry += 1;
    }

    fn get(&self, ch: char) -> usize {
        *self.0.get(&ch).unwrap_or(&0)
    }
}

#[derive(Debug)]
pub struct Policy {
    min: usize,
    max: usize,
    ch: char,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Policy, String)> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut split = s.split(": ");
            let policy = split.next().unwrap();
            let password = split.next().unwrap();

            let mut split = policy.split(' ');
            let range = split.next().unwrap();
            let ch = split.next().unwrap().chars().next().unwrap();

            let mut split = range.split('-');
            let min = split.next().unwrap().parse().unwrap();
            let max = split.next().unwrap().parse().unwrap();
            let policy = Policy { min, max, ch };

            (policy, password.to_string())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Policy, String)]) -> usize {
    let mut valid = 0;

    for (policy, password) in input {
        let counter = Counter::new(&password);
        let count = counter.get(policy.ch);

        if count >= policy.min && count <= policy.max {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Policy, String)]) -> usize {
    let mut valid = 0;

    for (policy, password) in input {
        let password = password.as_bytes();
        let first = password[policy.min - 1] == policy.ch as u8;
        let second = password[policy.max - 1] == policy.ch as u8;

        if first ^ second {
            valid += 1;
        }
    }

    valid
}
