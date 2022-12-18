use std::{collections::HashSet, hash::Hash};

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.split('\n').map(|line| {
        if line.starts_with("noop") {
            Instruction::Noop
        } else {
            let (_, right) = line.split_at(5);
            Instruction::Addx(right.parse().unwrap())
        }
    })
}

#[aoc(day10, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let instructions = instructions(input);
    let mut reg_x = 1;
    let mut cycles = Vec::new();

    for instr in instructions {
        cycles.push(Instruction::Noop);

        let Instruction::Addx(_) = instr else {
            continue;
        };

        cycles.push(instr);
    }

    let mut signal_strength = 0;

    for (i, instr) in cycles.iter().enumerate() {
        if i == 19 || (i > 19 && (i - 19) % 40 == 0) {
            signal_strength += reg_x * (i + 1) as i32;
        }

        let Instruction::Addx(val) = instr else {
            continue;
        };

        reg_x += val;
    }

    signal_strength
}

#[aoc(day10, part2, Chars)]
pub fn part2_chars(input: &str) -> String {
    let mut s = String::with_capacity(241);
    s.push('\n');

    let instructions = instructions(input);
    let mut reg_x = 1;
    let mut cycles = Vec::new();
    let mut pixels = HashSet::new();

    for instr in instructions {
        cycles.push(Instruction::Noop);

        let Instruction::Addx(_) = instr else {
            continue;
        };

        cycles.push(instr);
    }

    for (crt_i, instr) in cycles.iter().enumerate() {
        if (reg_x - 1..reg_x + 2).contains(&((crt_i % 40) as i32)) {
            pixels.insert(crt_i);
        }

        if let Instruction::Addx(val) = instr {
            reg_x += val;
        }
    }

    let mut s = String::new();

    s.push('\n');

    for i in 0..240 {
        s.push(if pixels.contains(&i) { '#' } else { '.' });

        if (i + 1) % 40 == 0 {
            s.push('\n');
        }
    }
    s
}
