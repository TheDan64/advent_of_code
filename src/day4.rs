use aoc_runner_derive::aoc;

fn pairs<'s>(input: &'s str) -> impl Iterator<Item = ((u8, u8), (u8, u8))> + 's {
    input.split('\n').map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let (lstart, lend) = left.split_once('-').unwrap();
        let (rstart, rend) = right.split_once('-').unwrap();
        let lstart = lstart.parse::<u8>().unwrap();
        let lend = lend.parse::<u8>().unwrap();
        let rstart = rstart.parse::<u8>().unwrap();
        let rend = rend.parse::<u8>().unwrap();

        ((lstart, lend), (rstart, rend))
    })
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    pairs(input)
        .map(|(left, right)| {
            if (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1)
            {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}

fn overlap(min1: u8, max1: u8, min2: u8, max2: u8) -> u8 {
    0.max(match max1.min(max2).checked_sub(min1.max(min2)) {
        Some(val) => val + 1,
        None => 0,
    })
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    pairs(input)
        .map(|(left, right)| {
            let overlap = overlap(left.0, left.1, right.0, right.1);

            if overlap > 0 {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}
