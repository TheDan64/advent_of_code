use aoc_runner_derive::aoc;

fn elves<'s>(input: &'s str) -> impl Iterator<Item = u32> + 's {
    input.split("\n\n").map(|elf| {
        elf.split('\n')
            .map(|cal| cal.parse::<u32>().unwrap())
            .sum::<u32>()
    })
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    elves(input).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut elves: Vec<_> = elves(input).collect();

    elves.sort_unstable();
    elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
}
