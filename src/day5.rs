use aoc_runner_derive::aoc;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    static ref STACKS_RE: Regex = Regex::new(r"(\[([A-Z])\]\s)|(\s{4})").unwrap();
}

struct Move {
    num: u8,
    from: u8,
    to: u8,
}

fn stacks<'s>(input: &'s str) -> (Vec<Vec<&'s str>>, impl Iterator<Item = Move> + 's) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let num_stacks = stacks[stacks.len() - 2..stacks.len() - 1]
        .parse::<usize>()
        .unwrap();
    let stack_chunks = STACKS_RE.captures_iter(stacks).chunks(num_stacks);
    let mut stacks = vec![Vec::new(); num_stacks];

    for chunk in &stack_chunks {
        for (i, cap) in chunk.enumerate() {
            if let Some(letter) = cap.get(2) {
                stacks[i].insert(0, letter.as_str());
            }
        }
    }

    let instructions = MOVE_RE.captures_iter(instructions).map(|cap| Move {
        num: cap[1].parse::<u8>().unwrap(),
        from: cap[2].parse::<u8>().unwrap(),
        to: cap[3].parse::<u8>().unwrap(),
    });

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = stacks(input);

    for mut instruction in moves {
        while instruction.num > 0 {
            instruction.num -= 1;
            let letter = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].push(letter);
        }
    }

    stacks.iter().map(|vec| *vec.last().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = stacks(input);

    for mut instruction in moves {
        let insert_at = stacks[instruction.to as usize - 1].len();

        while instruction.num > 0 {
            instruction.num -= 1;
            let letter = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].insert(insert_at, letter);
        }
    }

    stacks.iter().map(|vec| *vec.last().unwrap()).collect()
}
