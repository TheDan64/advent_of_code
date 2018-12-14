extern crate ring_queue;

use std::collections::HashMap;

use ring_queue::Ring;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> (usize, usize) {
    let mut nums = input.split(' ');
    let players = nums.next().unwrap().parse::<usize>().unwrap();
    let last_marble_points = nums.nth(5).unwrap().parse::<usize>().unwrap();

    (players, last_marble_points)
}

#[aoc(day9, part1, Chars)]
pub fn part1_chars(&(players, last_marble_points): &(usize, usize)) -> usize {
    let mut player_scores = HashMap::with_capacity(players);
    let mut marbles = Ring::with_capacity(last_marble_points);

    marbles.push(0);

    for i in 1..=last_marble_points {
        let player_id = (i - 1) % players + 1;
        let player_score = player_scores.entry(player_id).or_insert(0);

        if i % 23 == 0 {
            marbles.rotate(7);

            *player_score += i + marbles.pop().unwrap();

            marbles.rotate(-1);

            continue;
        }

        marbles.rotate(-1);
        marbles.push(i);
    }

    *player_scores.values().max().unwrap_or(&0)
}

#[aoc(day9, part2, Chars)]
pub fn part2_chars(values: &(usize, usize)) -> usize {
    let &(players, mut last_marble_points) = values;

    last_marble_points *= 100;

    // ~0.45s to go through 7197500 marbles!
    part1_chars(&(players, last_marble_points))
}
