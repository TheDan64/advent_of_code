struct Position {
    x: usize,
    y: usize,
}

struct Slope {
    right: usize,
    down: usize,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes())
        .map(ToOwned::to_owned)
        .collect()
}

fn calc_trees_hit(slope: &Slope, rows: &[Vec<u8>]) -> usize {
    let mut trees_hit = 0;
    let mut pos = Position { x: 0, y: 0 };
    let map_width = rows[0].len();
    let map_height = rows.len();

    while pos.y < map_height {
        pos.x += slope.right;
        pos.y += slope.down;

        if pos.y >= map_height {
            break;
        }

        if pos.x >= map_width {
            pos.x -= map_width;
        }

        if rows[pos.y][pos.x] == b'#' {
            trees_hit += 1;
        }
    }

    trees_hit
}

#[aoc(day3, part1)]
pub fn part1(rows: &[Vec<u8>]) -> usize {
    calc_trees_hit(&Slope { right: 3, down: 1 }, rows)
}

#[aoc(day3, part2)]
pub fn part2(rows: &[Vec<u8>]) -> usize {
    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes.iter().map(|slope| calc_trees_hit(slope, rows)).product()
}
