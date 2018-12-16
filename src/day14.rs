extern crate subslice;

use std::fmt::{self, Display, Formatter};
use subslice::bmh;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (Vec<u8>, usize) {
    let digits = input.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let num = input.parse::<usize>().unwrap();

    (digits, num)
}

#[derive(Debug, PartialEq, Eq)]
pub struct DisplayableArray10([u8; 10]);

impl Display for DisplayableArray10 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4],
               self.0[5], self.0[6], self.0[7], self.0[8], self.0[9])
    }
}

fn calculate_scores(digits: &Vec<u8>) -> Vec<u8> {
    let mut scores = Vec::with_capacity(1043826688);
    let mut elf1_index = 0;
    let mut elf2_index = 1;

    scores.push(3);
    scores.push(7);

    while scores.len() <= digits.len() || &scores[scores.len() - digits.len()..] != digits.as_slice() {
        let score = (scores[elf1_index] + scores[elf2_index]) as u8;
        let (ldigit, rdigit) = if score > 9 {
            (1, Some(score - 10))
        } else {
            (score, None)
        };

        scores.push(ldigit);

        if let Some(digit) = rdigit {
            scores.push(digit);
        }

        let elf1_move = elf1_index + scores[elf1_index] as usize + 1;
        let elf2_move = elf2_index + scores[elf2_index] as usize + 1;

        // Turns out branching this way is ~4ms faster than always
        // applying modulus
        elf1_index = if elf1_move >= scores.len() {
            elf1_move % scores.len()
        } else {
            elf1_move
        };

        elf2_index = if elf2_move >= scores.len() {
            elf2_move % scores.len()
        } else {
            elf2_move
        };
    }

    scores
}

#[aoc(day14, part1, Chars)]
pub fn part1_chars((digits, num_recipes): &(Vec<u8>, usize)) -> DisplayableArray10 {
    let scores = calculate_scores(&digits);
    let mut array = [0; 10];
    let end_slice = num_recipes + 10;

    array.copy_from_slice(&scores[*num_recipes..end_slice]);

    DisplayableArray10(array)
}

#[aoc(day14, part2, Chars)]
pub fn part2_chars((digits, _): &(Vec<u8>, usize)) -> usize {
    let scores = calculate_scores(&digits);
    let index = bmh::find(&scores, &digits).unwrap_or(0);

    scores[..index].len()
}
