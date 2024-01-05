use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let line_iter = input.split('\n');
    let mut sum = 0;

    for line in line_iter {
        let (game, data) = line.split_once(": ").unwrap();
        let game_id = game.split_once("Game ").unwrap().1.parse::<u32>().unwrap();
        let cube_iter = data.split([',', ';']);
        let mut invalid = false;

        for cube in cube_iter {
            let (digit, color) = cube.trim().split_once(' ').unwrap();
            let digit = digit.parse::<u32>().unwrap();

            match color {
                "green" if digit > 13 => invalid = true,
                "red" if digit > 12 => invalid = true,
                "blue" if digit > 14 => invalid = true,
                _ => (),
            }
        }

        if !invalid {
            sum += game_id;
        }
    }

    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let line_iter = input.split('\n');
    let mut sum = 0;

    for line in line_iter {
        let (_game, data) = line.split_once(": ").unwrap();
        let cube_iter = data.split([',', ';']);

        let mut reds = Vec::new();
        let mut greens = Vec::new();
        let mut blues = Vec::new();

        for cube in cube_iter {
            let (digit, color) = cube.trim().split_once(' ').unwrap();
            let digit = digit.parse::<u32>().unwrap();

            match color {
                "green" => greens.push(digit),
                "red" => reds.push(digit),
                "blue" => blues.push(digit),
                _ => (),
            }
        }

        let red_max = reds.iter().max().unwrap();
        let green_max = greens.iter().max().unwrap();
        let blue_max = blues.iter().max().unwrap();

        sum += *red_max * *green_max * *blue_max;
    }

    sum
}
