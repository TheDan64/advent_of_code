fn cmp_chars(lhs: char, rhs: char) -> bool {
    (lhs.is_lowercase() && lhs.to_uppercase().next().unwrap() == rhs) ||
    (lhs.is_uppercase() && lhs.to_lowercase().next().unwrap() == rhs)
}

fn react(input: &str) -> String {
    let mut iter = input.chars().peekable();
    let mut skip_next = false;
    let mut output = String::new();

    while let Some(ch) = iter.next() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if let Some(next_ch) = iter.peek() {
            if cmp_chars(ch, *next_ch) {
                skip_next = true;
                continue;
            }
        }

        output.push(ch);
    }

    output
}

#[aoc(day5, part1, Chars)]
pub fn part1_chars(input: &str) -> usize {
    let mut reduced = react(input);

    loop {
        let next_reduced = react(&reduced);

        if reduced == next_reduced {
            break reduced.len();
        }

        reduced = next_reduced;
    }
}

#[aoc(day5, part2, Chars)]
pub fn part2_chars(input: &str) -> usize {
    let char_iter = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut smallest = None;

    for ch in char_iter {
        let new_input: String = input.chars()
            .filter(|&chr| chr != ch && chr != ch.to_uppercase().next().unwrap())
            .collect();
        let reduced = part1_chars(&new_input);

        if smallest.is_none() || smallest.unwrap() > reduced {
            smallest = Some(reduced);
        }
    }

    smallest.unwrap()
}
