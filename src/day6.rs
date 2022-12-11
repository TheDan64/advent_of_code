use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

fn split_at_start_of_packet(input: &str, distinct: usize) -> (&str, &str) {
    for (i, _ch) in input.chars().enumerate() {
        let s = &input[i..i + distinct];

        if HashSet::<char>::from_iter(s.chars()).len() == distinct {
            return input.split_at(i + distinct);
        }
    }

    unreachable!()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    split_at_start_of_packet(input, 4).0.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (start, end) = split_at_start_of_packet(input, 4);
    let (marker, _) = split_at_start_of_packet(end, 14);
    start.len() + marker.len()
}
