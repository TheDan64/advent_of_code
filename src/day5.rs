use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
enum FrontBack {
    Back,
    Front,
}

#[derive(Clone, Copy)]
enum LeftRight {
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}

type SeatId = usize;

impl Seat {
    fn id(&self) -> SeatId {
        self.row * 8 + self.column
    }
}

pub struct BoardingPass {
    fb: ArrayVec<[FrontBack; 7]>,
    lr: ArrayVec<[LeftRight; 3]>,
}

impl BoardingPass {
    fn seat(&self) -> Seat {
        let (mut row_min, mut row_max) = (0, 127);
        let (mut column_min, mut column_max) = (0, 7);

        for fb in &self.fb {
            match fb {
                FrontBack::Front => row_max -= (row_max - row_min) / 2 + 1,
                FrontBack::Back => row_min += (row_max - row_min) / 2 + 1,
            }
        }

        for lr in &self.lr {
            match lr {
                LeftRight::Left => column_max -= (column_max - column_min) / 2 + 1,
                LeftRight::Right => column_min += (column_max - column_min) / 2 + 1,
            }
        }

        let row = match self.fb[6] {
            FrontBack::Front => row_min,
            FrontBack::Back => row_max,
        };
        let column = match self.lr[2] {
            LeftRight::Left => column_min,
            LeftRight::Right => column_max,
        };

        Seat { row, column }
    }
}

impl FromStr for BoardingPass {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let fb: ArrayVec<[_; 7]> = bytes[0..7].iter().map(|ch| if *ch == b'F' {
            FrontBack::Front
        } else {
            FrontBack::Back
        }).collect();
        let lr: ArrayVec<[_; 3]> = bytes[7..10].iter().map(|ch| if *ch == b'L' {
            LeftRight::Left
        } else {
            LeftRight::Right
        }).collect();

        Ok(BoardingPass { fb, lr })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    let lines = input.split('\n').filter(|s| !s.is_empty());

    lines.map(|s| s.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(passes: &[BoardingPass]) -> usize {
    passes.iter().map(|p| p.seat().id()).max().unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn part2(passes: &[BoardingPass]) -> usize {
    let all_ids: HashSet<_> = (0..=911usize).collect();
    let present_ids: HashSet<_> = passes.iter().map(|p| p.seat().id()).collect();
    let remaining_ids: HashSet<_> = all_ids.difference(&present_ids).collect();

    for id in remaining_ids.iter().copied() {
        let one_less_id = id.checked_sub(1).and_then(|id| remaining_ids.get(&id));
        let one_more_id = remaining_ids.get(&(*id + 1));

        if one_less_id.is_none() && one_more_id.is_none() {
            return *id
        }
    }

    unreachable!()
}
