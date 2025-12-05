use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Coord = (usize, usize);
type Input = (usize, usize, HashSet<Coord>);

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let grid_x = input.lines().next().unwrap().len();
    let grid_y = input.lines().count();
    let mut map = HashSet::with_capacity(grid_x * grid_y);

    for (y, row) in input.lines().enumerate() {
        for (x, item) in row.chars().enumerate() {
            if item == '@' {
                map.insert((x, y));
            }
        }
    }

    (grid_x, grid_y, map)
}

fn rolls_to_remove(grid_x: usize, grid_y: usize, map: &HashSet<Coord>) -> Vec<Coord> {
    let mut to_remove = Vec::new();

    for y in 0..grid_y {
        for x in 0..grid_x {
            if !map.contains(&(x, y)) {
                continue;
            }

            let mut count = 0;

            for offset_x in [x.checked_sub(1), Some(x), Some(x + 1)] {
                let Some(offset_x) = offset_x else {
                    continue;
                };

                for offset_y in [y.checked_sub(1), Some(y), Some(y + 1)] {
                    let Some(offset_y) = offset_y else {
                        continue;
                    };

                    if offset_x == x && offset_y == y {
                        continue;
                    }

                    if map.contains(&(offset_x, offset_y)) {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                to_remove.push((x, y));
            }
        }
    }

    to_remove
}

#[aoc(day4, part1)]
pub fn part1((grid_x, grid_y, map): &Input) -> usize {
    rolls_to_remove(*grid_x, *grid_y, map).len()
}

#[aoc(day4, part2)]
pub fn part2((grid_x, grid_y, map): &Input) -> usize {
    let mut map = map.clone();
    let mut total = 0;

    loop {
        let to_remove = rolls_to_remove(*grid_x, *grid_y, &map);

        if to_remove.is_empty() {
            break;
        }

        total += to_remove.len();

        for coord in to_remove {
            map.remove(&coord);
        }
    }

    total
}
