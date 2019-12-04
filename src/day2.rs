fn process(values: &[usize], noun: usize, verb: usize) -> Vec<usize> {
    let mut values = values.to_vec();

    values[1] = noun;
    values[2] = verb;

    let mut skip = 0;

    for i in 0..values.len() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match values[i] {
            1 => {
                let (x, y, dest) = (values[i + 1], values[i + 2], values[i + 3]);
                values[dest as usize] = values[x as usize] + values[y as usize];
                skip += 3;
            },
            2 => {
                let (x, y, dest) = (values[i + 1], values[i + 2], values[i + 3]);
                values[dest as usize] = values[x as usize] * values[y as usize];
                skip += 3;
            },
            99 => break,
            _ => unreachable!(),
        }
    }

    values
}

#[aoc(day2, part1, Chars)]
pub fn part1_chars(input: &str) -> usize {
    let values: Vec<_> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    process(&values, 12, 2)[0]
}

#[aoc(day2, part2, Chars)]
pub fn part2_chars(input: &str) -> usize {
    let values: Vec<_> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            if process(&values, noun, verb)[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}
