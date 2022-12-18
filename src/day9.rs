use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use itertools::repeat_n;

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

fn dirs(input: &str) -> impl Iterator<Item = Dir> + '_ {
    input
        .split('\n')
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let left = match left {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => unreachable!(),
            };
            let right = right.parse::<usize>().unwrap();

            repeat_n(left, right)
        })
        .flatten()
}

fn move_behind(dir: Dir, (front_x, front_y): (i32, i32), (back_x, back_y): (&mut i32, &mut i32)) {
    match dir {
        Dir::Up => {
            *back_x = front_x;
            *back_y = front_y - 1;
        }
        Dir::Down => {
            *back_x = front_x;
            *back_y = front_y + 1;
        }
        Dir::Left => {
            *back_x = front_x + 1;
            *back_y = front_y;
        }
        Dir::Right => {
            *back_x = front_x - 1;
            *back_y = front_y;
        }
        Dir::UpRight => {
            *back_x = front_x - 1;
            *back_y = front_y - 1;
        }
        Dir::DownRight => {
            *back_x = front_x - 1;
            *back_y = front_y + 1;
        }
        Dir::UpLeft => {
            *back_x = front_x + 1;
            *back_y = front_y - 1;
        }
        Dir::DownLeft => {
            *back_x = front_x + 1;
            *back_y = front_y + 1;
        }
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let (mut head_x, mut head_y) = (0i32, 0i32);
    let (mut tail_x, mut tail_y) = (0, 0);

    visited.insert((0, 0));

    for dir in dirs(input) {
        match dir {
            Dir::Up => head_y += 1,
            Dir::Down => head_y -= 1,
            Dir::Left => head_x -= 1,
            Dir::Right => head_x += 1,
            _ => unreachable!(),
        }

        let dist = (((head_x - tail_x).pow(2) + (head_y - tail_y).pow(2)) as f64)
            .sqrt()
            .floor() as u32;

        if dist < 2 {
            continue;
        }

        move_behind(dir, (head_x, head_y), (&mut tail_x, &mut tail_y));

        visited.insert((tail_x, tail_y));
    }

    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    const NUM_TAILS: usize = 9;
    //     let input = "R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2";
    //     let input = "R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20";

    let mut visited = HashSet::new();
    let (mut head_x, mut head_y) = (0i32, 0i32);
    let mut tail_pos = HashMap::with_capacity(NUM_TAILS);

    visited.insert((0, 0));

    for dir in dirs(input) {
        match dir {
            Dir::Up => head_y += 1,
            Dir::Down => head_y -= 1,
            Dir::Left => head_x -= 1,
            Dir::Right => head_x += 1,
            _ => unreachable!(),
        }

        for tail_i in 0..NUM_TAILS {
            let (last_x, last_y) = match tail_i.checked_sub(1) {
                Some(last_i) => tail_pos[&last_i],
                None => (head_x, head_y),
            };
            let (tail_x, tail_y) = tail_pos.entry(tail_i).or_default();
            let dist = (((last_x - *tail_x).pow(2) + (last_y - *tail_y).pow(2)) as f64)
                .sqrt()
                .floor() as u32;

            if dist < 2 {
                break;
            }

            let diff_x = last_x - *tail_x;
            let diff_y = last_y - *tail_y;
            let dir = if diff_x >= 0 && diff_y >= 0 {
                if diff_x > diff_y {
                    Dir::Right
                } else if diff_x == diff_y {
                    Dir::UpRight
                } else {
                    Dir::Up
                }
            } else if diff_x >= 0 && diff_y <= 0 {
                if diff_x > diff_y.abs() {
                    Dir::Right
                } else if diff_x == diff_y {
                    Dir::DownRight
                } else {
                    Dir::Down
                }
            } else if diff_x <= 0 && diff_y >= 0 {
                if diff_x.abs() > diff_y {
                    Dir::Left
                } else if diff_x == diff_y {
                    Dir::UpLeft
                } else {
                    Dir::Up
                }
            } else if diff_x <= 0 && diff_y <= 0 {
                if diff_x < diff_y {
                    Dir::Left
                } else if diff_x == diff_y {
                    Dir::DownLeft
                } else {
                    Dir::Down
                }
            } else {
                unreachable!()
            };

            move_behind(dir, (last_x, last_y), (tail_x, tail_y));

            if tail_i == NUM_TAILS - 1 {
                visited.insert((*tail_x, *tail_y));
            }
        }

        let min_x = tail_pos
            .values()
            .map(|(x, _)| *x)
            .min()
            .unwrap()
            .min(head_x);
        let max_x = tail_pos
            .values()
            .map(|(x, _)| *x)
            .max()
            .unwrap()
            .max(head_x);
        let min_y = tail_pos
            .values()
            .map(|(_, y)| *y)
            .min()
            .unwrap()
            .min(head_y);
        let max_y = tail_pos
            .values()
            .map(|(_, y)| *y)
            .max()
            .unwrap()
            .max(head_y);

        // println!("{dir:?}");
        // for y in (min_y..=max_y).rev() {
        //     for x in min_x..=max_x {
        //         if (x, y) == (head_x, head_y) {
        //             print!("H");
        //         } else if let Some(i) = tail_pos
        //             .iter()
        //             .filter_map(|(i, (ix, iy))| if (*ix, *iy) == (x, y) { Some(i) } else { None })
        //             .max()
        //         {
        //             print!("{}", i + 1);
        //         } else {
        //             print!(".");
        //         }
        //     }

        //     println!();
        // }
    }

    visited.len()
}
