use aoc_runner_derive::aoc;

fn generator(input: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    input.split(',').map(|l| {
        let mut iter = l.split('-');

        (iter.next().unwrap(), iter.next().unwrap())
    })
}

fn is_valid(id: &str) -> bool {
    // if odd
    if id.len() % 2 == 1 {
        return true;
    }

    let (left, right) = id.split_at(id.len() / 2);

    left != right
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for (id1, id2) in generator(input) {
        let start = id1.parse::<u64>().unwrap();
        let end = id2.parse::<u64>().unwrap();

        for num in start..=end {
            let id = num.to_string();

            if !is_valid(&id) {
                sum += num;
            }
        }
    }

    sum
}

// Repeated patterns are not allowed:
//  - all digits the same (e.g., 111111)
//  - repeats of 2 (e.g., 121212, 45454545)
//  - repeats of 3 (e.g., 123123, 987987987)
//  - repeats of 4 (e.g., 12341234, 98769876)
//  - repeats of 5 (e.g., 1234512345, 9876598765)
//  - up to 10 digits are expected inputs
pub fn is_valid_part2(id: &str) -> bool {
    // Single digits have no repeats
    if id.len() == 1 {
        return true;
    }

    let first_char = id.chars().next().unwrap();

    // all digits the same
    if id.chars().all(|ch| ch == first_char) {
        return false;
    }

    // any odd != 9 is auto valid
    if id.len() != 9 && id.len() % 2 == 1 {
        return true;
    }

    let first_two = &id[..2];

    // Check repeats of 2
    if [4, 6, 8, 10].contains(&id.len())
        && id
            .char_indices()
            .skip(2)
            .all(|(idx, ch)| first_two.chars().nth(idx % 2) == Some(ch))
    {
        return false;
    }

    // Check repeats of 3
    if [6, 9].contains(&id.len())
        && id
            .char_indices()
            .skip(3)
            .all(|(idx, ch)| id[..3].chars().nth(idx % 3) == Some(ch))
    {
        return false;
    }

    // Check repeats of 4
    if id.len() == 8 {
        let (left, right) = id.split_at(4);

        if left == right {
            return false;
        }
    }

    // Check repeats of 5
    if id.len() == 10 {
        let (left, right) = id.split_at(5);

        if left == right {
            return false;
        }
    }

    true
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let mut sum = 0;

    for (id1, id2) in generator(input) {
        let start = id1.parse::<u64>().unwrap();
        let end = id2.parse::<u64>().unwrap();

        for num in start..=end {
            let id = num.to_string();

            if !is_valid_part2(&id) {
                sum += num;
            }
        }
    }

    sum
}

#[test]
fn test_day2_is_valid() {
    assert!(is_valid("111"));
    assert!(is_valid("1"));
    assert!(!is_valid("1010"));
}

#[test]
fn test_day2_is_valid_part2() {
    assert!(is_valid_part2("12"));
    assert!(!is_valid_part2("11111"));
    assert!(is_valid_part2("111112"));
    assert!(!is_valid_part2("10101010"));
    assert!(is_valid_part2("101010102"));
    assert!(!is_valid_part2("123123"));
    assert!(is_valid_part2("1231234"));
    assert!(!is_valid_part2("824824824"));
    assert!(!is_valid_part2("98769876"));
}
