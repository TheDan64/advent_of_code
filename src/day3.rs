use std::collections::HashSet;
use std::iter::FromIterator;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn compartments<'s>(input: &'s str) -> impl Iterator<Item = (HashSet<char>, HashSet<char>)> + 's {
    input.split('\n').map(|rucksack| {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        (
            HashSet::from_iter(left.chars()),
            HashSet::from_iter(right.chars()),
        )
    })
}

fn score(ch: &char) -> u64 {
    match ch {
        'a'..='z' => *ch as u64 - 96,
        'A'..='Z' => *ch as u64 - 38,
        _ => unreachable!(),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    compartments(input)
        .map(|(left, right)| left.intersection(&right).map(score).sum::<u64>())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    compartments(input)
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut sets = group.map(|(s1, s2)| &s1 | &s2);
            sets.next()
                .map(|set| {
                    sets.fold(set, |set1, set2| &set1 & &set2)
                        .iter()
                        .map(score)
                        .sum::<u64>()
                })
                .unwrap()
        })
        .sum()
}
