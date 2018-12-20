use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Point {
    Source { x: u32, y: u32, dist: u32 },
    Edge { x: u32, y: u32, dist: u32 },
    Equidistant,
}

impl Point {
    fn is_edge(&self) -> bool {
        match self {
            Point::Edge { .. } => true,
            _ => false,
        }
    }

    fn x(&self) -> u32 {
        match self {
            Point::Edge { x, .. } => *x,
            Point::Source { x, .. } => *x,
            _ => panic!("No x coordinate on Equidistant"),
        }
    }

    fn y(&self) -> u32 {
        match self {
            Point::Edge { y, .. } => *y,
            Point::Source { y, .. } => *y,
            _ => panic!("No x coordinate on Equidistant"),
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    input
        .split('\n')
        .filter(|s| s.len() != 0)
        .map(|line| {
            let mut nums = line.split(", ");
            let x = nums.next().unwrap().parse::<u32>().unwrap();
            let y = nums.next().unwrap().parse::<u32>().unwrap();

            (x, y)
        })
        .collect()
}

#[aoc(day6, part1, Chars)]
pub fn part1_chars(tuples: &[(u32, u32)]) -> u32 {
    let largest_x = tuples.iter().max_by_key(|(x, _)| x).unwrap().0;
    let largest_y = tuples.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut points = vec![Point::Equidistant; (largest_x + 1) as usize * (largest_y + 1) as usize];

    // Ugh, wonder if there is a less complex way to calc this
    // than 50 * (largest_x + 1) * (largest_y + 1)
    for (i, point) in points.iter_mut().enumerate() {
        let i_x = i % (largest_x + 1) as usize;
        let i_y = i / (largest_x + 1) as usize;

        let mut smallest_x = 0;
        let mut smallest_y = 0;
        let mut smallest_dist = u32::max_value();
        let mut equidistant = false;

        for &(x, y) in tuples {
            let dist = ((x as i32 - i_x as i32).abs() + (y as i32 - i_y as i32).abs()) as u32;

            if dist < smallest_dist {
                smallest_dist = dist;
                smallest_x = x;
                smallest_y = y;
                equidistant = false;
            } else if dist == smallest_dist {
                equidistant = true;
            }
        }

        *point = if equidistant {
            Point::Equidistant
        } else if i_x == 0 || i_y == 0 || i_x as u32 == largest_x || i_y as u32 == largest_y {
            Point::Edge { x: smallest_x, y: smallest_y, dist: smallest_dist }
        } else {
            Point::Source { x: smallest_x, y: smallest_y, dist: smallest_dist }
        }
    }

    // Done Setup

    let inifinite_areas: HashSet<(u32, u32)> = points.iter()
        .filter(|point| point.is_edge())
        .map(|point| (point.x(), point.y()))
        .collect();
    let mut areas = HashMap::new();

    for point in points {
        match point {
            Point::Source { x, y, .. } => {
                if inifinite_areas.contains(&(x, y)) {
                    continue;
                }

                *areas.entry((x, y)).or_insert(0) += 1;
            },
            Point::Edge { .. } => {},
            _ => {},
        }
    }

    *areas.values().max().expect("max val")
}

#[aoc(day6, part2, Chars)]
pub fn part2_chars(points: &[(u32, u32)]) -> usize {
    let smallest_x = points.iter().min_by_key(|(x, _)| x).unwrap().0;
    let smallest_y = points.iter().min_by_key(|(_, y)| y).unwrap().1;
    let largest_x = points.iter().max_by_key(|(x, _)| x).unwrap().0;
    let largest_y = points.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut in_region = HashSet::new();

    // Yay, triple for loop...
    for x in smallest_x..=largest_x {
        for y in smallest_y..=largest_y {
            let mut dist_sum = 0;

            for &(p_x, p_y) in points {
                let dist = ((x as i32 - p_x as i32).abs() + (y as i32 - p_y as i32).abs()) as u32;

                dist_sum += dist;

                if dist_sum >= 10_000 {
                    break;
                }
            }

            if dist_sum < 10_000 {
                in_region.insert((x, y));
            }
        }
    }

    in_region.len()
}
