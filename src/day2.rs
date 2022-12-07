use aoc_runner_derive::aoc;
use std::cmp::Ordering::{self, *};
use RPS::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Equal,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Greater,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Less,
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum XYZ {
    X,
    Y,
    Z,
}

impl From<XYZ> for RPS {
    fn from(xyz: XYZ) -> Self {
        match xyz {
            XYZ::X => Rock,
            XYZ::Y => Paper,
            XYZ::Z => Scissors,
        }
    }
}

impl From<XYZ> for Ordering {
    fn from(xyz: XYZ) -> Self {
        match xyz {
            XYZ::X => Greater,
            XYZ::Y => Equal,
            XYZ::Z => Less,
        }
    }
}

fn rounds<'s>(input: &'s str) -> impl Iterator<Item = (RPS, XYZ)> + 's {
    input.split('\n').map(|round| {
        let (left, right) = round.split_once(' ').unwrap();
        let left = match left {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => unreachable!(),
        };
        let right = match right {
            "X" => XYZ::X,
            "Y" => XYZ::Y,
            "Z" => XYZ::Z,
            _ => unreachable!(),
        };

        (left, right)
    })
}

fn score(left: RPS, right: RPS) -> u32 {
    if right > left {
        6 + right.score()
    } else if right == left {
        3 + right.score()
    } else {
        right.score()
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    rounds(input)
        .map(|(left, right)| score(left, RPS::from(right)))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    rounds(input)
        .map(|(left, right)| {
            let result = Ordering::from(right);

            for hand in [Rock, Paper, Scissors] {
                if left.cmp(&hand) == result {
                    return score(left, hand);
                }
            }

            unreachable!()
        })
        .sum()
}
