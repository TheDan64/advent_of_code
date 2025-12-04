use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let banks = input.lines();
    let mut joltage = 0;

    for bank in banks {
        let mut greatest_pos = 0usize;
        let mut greatest = 0;

        for (pos, ch) in bank.chars().take(bank.len() - 1).enumerate() {
            let digit = ch.to_digit(10).unwrap();

            if digit > greatest {
                greatest_pos = pos;
                greatest = digit;
            }
        }

        let next_greatest = bank.chars().skip(greatest_pos + 1).max().unwrap();
        let next_greatest = next_greatest.to_digit(10).unwrap();

        joltage += greatest * 10 + next_greatest;
    }

    joltage
}

fn prune(batteries: &mut Vec<char>) -> String {
    loop {
        let len = batteries.len();

        if len == 12 {
            break;
        }

        let mut iter = batteries.iter_mut().enumerate().peekable();

        while let Some((pos, ch)) = iter.next() {
            let digit = ch.to_digit(10).unwrap();
            let Some((_, next_digit)) = iter.peek() else {
                batteries.pop();
                break;
            };

            let next_digit = next_digit.to_digit(10).unwrap();

            if digit < next_digit {
                batteries.remove(pos);
                break;
            }
        }
    }

    batteries.iter().copied().collect()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let banks = input.lines();
    let mut joltage = 0u64;

    for bank in banks {
        let mut batteries = bank.chars().collect::<Vec<_>>();
        let pruned = prune(&mut batteries);

        joltage += pruned.parse::<u64>().unwrap();
    }

    joltage
}

#[test]
fn test_day3_prune() {
    assert_eq!(
        prune(&mut "987654321111111".chars().collect()),
        "987654321111"
    );
    assert_eq!(
        prune(&mut "811111111111119".chars().collect()),
        "811111111119"
    );
    assert_eq!(
        prune(&mut "818181911112111".chars().collect()),
        "888911112111"
    );
    assert_eq!(
        prune(&mut "234234234234278".chars().collect()),
        "434234234278"
    );
}
