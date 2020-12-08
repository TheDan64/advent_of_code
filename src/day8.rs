use std::collections::HashSet;

use Instruction::*;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Noop(isize),
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let lines = input.split('\n').filter(|s| !s.is_empty());

    lines.map(|s| {
        let mut split = s.split(' ');
        let instr = split.next().unwrap();
        let val = split.next().unwrap().parse::<isize>().unwrap();

        match instr {
            "acc" => Acc(val),
            "jmp" => Jmp(val),
            "nop" => Noop(val),
            _ => unreachable!(),
        }

    }).collect()
}

fn get_final_acc(instructions: &[Instruction]) -> Result<isize, isize> {
    let mut i = 0;
    let mut acc = 0;
    let mut visited_instruction = HashSet::with_capacity(instructions.len());

    while i < instructions.len() {
        if visited_instruction.get(&i).is_some() {
            return Err(acc);
        }

        visited_instruction.insert(i);

        match instructions[i] {
            Acc(val) => acc += val,
            Jmp(val) => {
                i = ((i as isize) + val) as usize;
                continue;
            },
            Noop(_val) => {},
        }

        i += 1;
    }

    Ok(acc)
}

#[aoc(day8, part1)]
pub fn part1(instructions: &[Instruction]) -> isize {
    get_final_acc(instructions).unwrap_err()
}

fn swap(instr: &mut Instruction) {
    *instr = match instr {
        Acc(val) => Acc(*val),
        Jmp(val) => Noop(*val),
        Noop(val) => Jmp(*val),
    };
}

#[aoc(day8, part2)]
pub fn part2(instructions: &[Instruction]) -> isize {
    let mut instructions: Vec<_> = instructions.iter().copied().collect();

    for i in 0..instructions.len() {
        swap(&mut instructions[i]);

        if let Ok(acc) = get_final_acc(&instructions) {
            return acc;
        }

        swap(&mut instructions[i]);
    }

    unreachable!()
}
