fn fuel_req(num: i32) -> i32 {
    num / 3 - 2
}

#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let mut sum = 0;
    let num_strs = input.split('\n').filter(|s| !s.is_empty());

    for num_str in num_strs {
        sum += fuel_req(num_str.parse().unwrap());
    }

    sum
}

#[aoc(day1, part2, Chars)]
pub fn part2_chars(input: &str) -> i32 {
    let mut sum = 0;
    let num_strs = input.split('\n').filter(|s| !s.is_empty());

    for num_str in num_strs {
        let mut req = num_str.parse().unwrap();

        loop {
            req = fuel_req(req);

            if req <= 0 {
                break;
            }

            sum += req;
        }
    }

    sum
}
