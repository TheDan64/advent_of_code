use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FORMAT: Regex = Regex::new(r"position=<\s?(-?\d+), \s?(-?\d+)> velocity=<\s?(-?\d+), \s?(-?\d+)>").unwrap();
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    FORMAT.captures_iter(input).map(|cap| {
        let x = cap[1].parse::<i64>().unwrap();
        let y = cap[2].parse::<i64>().unwrap();
        let velocity_x = cap[3].parse::<i64>().unwrap();
        let velocity_y = cap[4].parse::<i64>().unwrap();

        Point {
            x, y, velocity_x, velocity_y,
        }
    }).collect()
}

fn print_points(points: &[Point]) -> String {
    let mut string = String::new();

    string.push('\n');

    let lowest_x = points.iter().min_by_key(|p| p.x).unwrap().x;
    let lowest_y = points.iter().min_by_key(|p| p.y).unwrap().y;
    let largest_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let largest_y = points.iter().max_by_key(|p| p.y).unwrap().y;
    let point_map: HashSet<_> = points.iter().map(|p| (p.x, p.y)).collect();

    for y in lowest_y..=largest_y {
        for x in lowest_x..=largest_x {
            if !point_map.contains(&(x, y)) {
                string.push('.');

                continue;
            }

            string.push('#');
        }

        string.push('\n');
    }

    string.push('\n');
    string
}

fn calc_area(points: &[Point]) -> u64 {
    let lowest_x = points.iter().min_by_key(|p| p.x).unwrap().x;
    let lowest_y = points.iter().min_by_key(|p| p.y).unwrap().y;
    let largest_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let largest_y = points.iter().max_by_key(|p| p.y).unwrap().y;

    ((largest_y - lowest_y) * (largest_x - lowest_x)) as u64
}

fn find_lowest_area_points(points: &[Point]) -> (i64, Vec<Point>) {
    let mut points2 = points.into_iter().map(|x| *x).collect::<Vec<Point>>();
    let mut variance = u64::max_value();
    let mut variance_i = 0;

    // Find minimum area
    for i in 1.. {
        // Update positions
        for mut point in points2.iter_mut() {
            point.x += point.velocity_x;
            point.y += point.velocity_y;
        }

        let var = calc_area(&points2);

        if var < variance {
            variance = var;
            variance_i = i;
        } else {
            break;
        }
    }

    let mut points3 = points.into_iter().map(|x| *x).collect::<Vec<Point>>();

    for mut point in points3.iter_mut() {
        point.x += point.velocity_x * variance_i;
        point.y += point.velocity_y * variance_i;
    }

    (variance_i, points3)
}

#[aoc(day10, part1, Chars)]
pub fn part1_chars(points: &[Point]) -> String {
    let (_, points) = find_lowest_area_points(points);

    print_points(&points)
}

#[aoc(day10, part2, Chars)]
pub fn part2_chars(points: &[Point]) -> i64 {
    let (i, _) = find_lowest_area_points(points);

    i
}
