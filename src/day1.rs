use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let iter = input.split('\n');
    let mut total = 0;

    for line in iter {
        let start_iter = line.chars();
        let end_iter = line.chars().rev();

        let mut first_digit = 0;
        let mut last_digit = 0;

        for ch in start_iter {
            if ch.is_numeric() {
                first_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        for ch in end_iter {
            if ch.is_numeric() {
                last_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        total += (first_digit * 10) + last_digit;
    }

    total
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let iter = input.split('\n');
    let mut total = 0;

    for line in iter {
        let start_iter = line.chars();
        let end_iter = line.chars().rev();

        let mut first_digit = 0;
        let mut first_digit_pos = None;
        let mut last_digit = 0;
        let mut last_digit_pos = None;

        // Look for the first digit 1 ... 9
        for (pos, ch) in start_iter.enumerate() {
            if ch.is_numeric() {
                first_digit = ch.to_digit(10).unwrap();
                first_digit_pos = Some(pos);
                break;
            }
        }

        // Look for the last digit 1 ... 9
        for (pos, ch) in end_iter.enumerate() {
            if ch.is_numeric() {
                // 1 is pos 4 going right to left
                // two1nine
                // pos going left to right
                // 8 - 4 - 1 = 3

                // abcone2threexyz
                // 15 - 8 - 1 = 6

                last_digit = ch.to_digit(10).unwrap();
                last_digit_pos = Some(line.len() - pos - 1);
                break;
            }
        }

        find_word_number(line, "zero", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "one", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "two", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "three", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "four", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "five", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "six", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "seven", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "eight", &mut first_digit, &mut first_digit_pos);
        find_word_number(line, "nine", &mut first_digit, &mut first_digit_pos);

        rfind_word_number(line, "zero", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "one", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "two", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "three", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "four", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "five", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "six", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "seven", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "eight", &mut last_digit, &mut last_digit_pos);
        rfind_word_number(line, "nine", &mut last_digit, &mut last_digit_pos);

        total += (first_digit * 10) + last_digit;
    }

    total
}

// "one", "two", ...
fn find_word_number(
    line: &str,
    word: &str,
    first_digit: &mut u32,
    first_digit_pos: &mut Option<usize>,
) {
    let digit = match word {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        e => unreachable!("{}", e),
    };

    if let Some(pos) = line.find(word) {
        match first_digit_pos {
            Some(first_pos) => {
                if pos < *first_pos {
                    *first_digit = digit;
                    *first_digit_pos = Some(pos);
                }
            }
            None => {
                *first_digit = digit;
                *first_digit_pos = Some(pos);
            }
        }
    }
}

fn rfind_word_number(
    line: &str,
    word: &str,
    last_digit: &mut u32,
    last_digit_pos: &mut Option<usize>,
) {
    let digit = match word {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        e => unreachable!("{}", e),
    };

    if let Some(pos) = line.rfind(word) {
        match last_digit_pos {
            Some(last_pos) => {
                if pos > *last_pos {
                    *last_digit = digit;
                    *last_digit_pos = Some(pos);
                }
            }
            None => {
                *last_digit = digit;
                *last_digit_pos = Some(pos);
            }
        }
    }
}
