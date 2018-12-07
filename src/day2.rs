use std::collections::HashMap;

#[aoc(day2, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let lines_iter = input
        .split('\n')
        .filter(|s| s.len() != 0);
    let mut twos = 0;
    let mut threes = 0;
    let mut count = HashMap::new();

    for line in lines_iter {
        let chars = line.chars();

        count.clear();

        let mut found_two = false;
        let mut found_three = false;

        for ch in chars {
            *count.entry(ch).or_insert(0) += 1;
        }

        for val in count.values() {
            if *val == 2 && !found_two {
                twos += 1;

                found_two = true;
            }

            if *val == 3 && !found_three {
                threes += 1;

                found_three = true;
            }
        }
    }

    twos * threes
}

fn remove_diff(lstr: &str, rstr: &str) -> String {
    lstr.chars()
        .zip(rstr.chars())
        .filter(|(lchar, rchar)| lchar == rchar)
        .map(|(lchar, _)| lchar)
        .collect()
}

#[aoc(day2, part2, Chars)]
pub fn part2_chars(input: &str) -> String {
    let mut lines: Vec<&str> = input
        .split('\n')
        .filter(|s| s.len() != 0)
        .collect();

    lines.sort();

    let iter1 = lines.iter();
    let iter2 = lines.iter().skip(1);
    let joined = iter1.zip(iter2);

    for (first, next) in joined {
        let mut diff = 0;

        for (ch1, ch2) in first.chars().zip(next.chars()) {
            if ch1 != ch2 {
                diff += 1;
            }
        }

        if diff == 1 {
            return remove_diff(first, next);
        }
    }

    unreachable!()
}
