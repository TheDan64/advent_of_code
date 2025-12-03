use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    let mut dial = 50;
    let mut password = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let step = num.trim().parse::<i16>().unwrap();

        match dir {
            "L" => dial -= step,
            "R" => dial += step,
            _ => continue,
        }

        dial %= 100;

        if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            password += 1;
        }
    }

    password
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i16 {
    let mut dial: i16 = 50;
    let mut password = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let step = num.trim().parse::<i16>().unwrap();
        let dial_before = dial;

        match dir {
            "L" => dial -= step,
            "R" => dial += step,
            _ => unreachable!(),
        }

        let lower = dial_before.min(dial);
        let upper = dial_before.max(dial);

        if dir == "L" {
            for num in lower..upper {
                if num % 100 == 0 {
                    password += 1;
                }
            }
        } else {
            for num in (lower + 1)..=upper {
                if num % 100 == 0 {
                    password += 1;
                }
            }
        }
    }

    password
}

#[test]
fn test_day1_part2() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    assert_eq!(part2(input), 6);
}
