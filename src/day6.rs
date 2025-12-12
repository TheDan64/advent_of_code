use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let columns = input
        .split('\n')
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .count();
    let rows = input.lines().count();
    let mut slots = vec![Vec::with_capacity(rows - 1); columns];
    let mut total = 0;

    for (row_idx, line) in input.lines().enumerate() {
        if row_idx < rows - 1 {
            for (col_idx, slot) in line.split(' ').filter(|s| !s.is_empty()).enumerate() {
                slots[col_idx].push(slot);
            }
        } else {
            for (col_idx, cmd) in line.split(' ').filter(|s| !s.is_empty()).enumerate() {
                let iter = slots[col_idx].iter().map(|s| s.parse::<usize>().unwrap());

                total += match cmd {
                    "*" => iter.product::<usize>(),
                    "+" => iter.sum::<usize>(),
                    _ => unreachable!(),
                }
            }
        }
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut column_widths = input
        .split('\n')
        .last()
        .unwrap()
        .split(['*', '+'])
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect::<Vec<_>>();
    let len = column_widths.len();

    // No extra space present for the last column
    column_widths[len - 1] += 1;

    let columns = input
        .split('\n')
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .count();
    let rows = input.lines().count();
    let mut slots = vec![Vec::with_capacity(rows - 1); columns];
    let mut total = 0;

    for (row_idx, line) in input.lines().enumerate() {
        if row_idx < rows - 1 {
            let mut col_idx = 0;
            let mut col_pos = 0;

            while let Some(width) = column_widths.get(col_idx) {
                let slot = &line[col_pos..col_pos + width];

                slots[col_idx].push(slot);

                col_pos += width + 1;
                col_idx += 1;
            }
        } else {
            for (col_idx, cmd) in line.split(' ').filter(|s| !s.is_empty()).enumerate() {
                let op = match cmd {
                    "*" => |a, b| a * b,
                    "+" => |a, b| a + b,
                    _ => unreachable!(),
                };
                let mut totals = vec![0; slots[col_idx][0].len()];

                for row in &slots[col_idx] {
                    for (i, ch) in row.chars().rev().enumerate() {
                        if ch == ' ' {
                            continue;
                        }

                        let val = ch.to_digit(10).unwrap() as usize;
                        totals[i] *= 10;
                        totals[i] += val;
                    }
                }

                let column_total = totals.into_iter().reduce(op).unwrap();

                total += column_total;
            }
        }
    }

    total
}
