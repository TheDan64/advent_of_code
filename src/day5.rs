use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u64 {
    let mut iter = input.split("\n\n");
    let ranges = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    let ids = iter
        .next()
        .unwrap()
        .lines()
        .map(|id| id.parse::<u64>().unwrap());

    let mut total = 0;

    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                // dbg!((id, range));
                total += 1;
                break;
            }
        }
    }

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u64 {
    let mut ranges = Vec::new();
    let mut iter = input.split("\n\n");
    let iter = iter.next().unwrap().lines().map(|line| {
        let mut parts = line.split('-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();
        (start, end)
    });

    'outer: for (mut start, end) in iter {
        if ranges.is_empty() {
            ranges.push((start, end));
            continue;
        }

        let mut i = usize::MAX;

        loop {
            i = i.wrapping_add(1);

            let Some(existing) = ranges.get_mut(i) else {
                break;
            };

            // Check for overlap
            if start < existing.0 && end >= existing.0 {
                let existing_start = existing.0;
                let existing_end = existing.1;

                ranges.insert(i, (start, existing_start - 1));

                start = existing_end + 1;
                // i += 1;
                continue;
            }

            if end > existing.1 && start <= existing.1 {
                start = existing.1 + 1;

                if start > end {
                    continue 'outer;
                }
            }

            if start >= existing.0 && end <= existing.1 {
                continue 'outer;
            }

            if end < existing.0 {
                ranges.insert(i, (start, end));
                continue 'outer;
            }
        }

        ranges.push((start, end));
    }

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

#[test]
fn test_day5_part2() {
    let input = "3-5
10-14
16-20
12-18

32";
    let input2 = "3-5
10-14
16-20
12-18
9-21

32";

    assert_eq!(part2(input), 14);
    assert_eq!(part2(input2), 16);
}
