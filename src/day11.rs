use std::fmt::{self, Display, Formatter};

const GRID_SIZE: usize = 300 * 300;

#[derive(Clone)]
pub struct Grid {
    grid: [FuelCell; GRID_SIZE],
}

impl Grid {
    fn select_3x3(&self, mut x: usize, mut y: usize) -> [FuelCell; 9] {
        // Center values
        x += 1;
        y += 1;

        let map_width = 300;

        let center_i = (y * map_width) + x;
        let tl = self.grid[center_i - map_width - 1];
        let tm = self.grid[center_i - map_width];
        let tr = self.grid[center_i - map_width + 1];

        let l = self.grid[center_i-1];
        let m = self.grid[center_i];
        let r = self.grid[center_i+1];

        let bl = self.grid[center_i + map_width - 1];
        let bm = self.grid[center_i + map_width];
        let br = self.grid[center_i + map_width + 1];

        [tl, tm, tr, l, m, r, bl, bm, br]
    }

    fn sum_sub_grid(&self, x: usize, y: usize, size: usize) -> i32 {
        let mut sum = 0;
        let map_width = 300;

        let tl_i = (y * map_width) + x;

        for offset in 0..size {
            let start = tl_i + offset * 300;
            let end = start + size - 1;

            sum += self.grid[end].power_level as i32;

            if x > 0 {
                sum -= self.grid[start - 1].power_level as i32;
            }
        }

        sum
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FuelCell {
    power_level: i16,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
    let grid_serial_number = input.parse::<isize>().unwrap();
    let mut grid = [FuelCell { power_level: 0 }; GRID_SIZE];

    for (i, fuel_cell) in grid.iter_mut().enumerate() {
        let x = i as isize % 300 + 1;
        let y = i as isize / 300 + 1;
        let rack_id = x + 10;
        let mut power_level = rack_id * y;

        power_level += grid_serial_number;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level -= 5;

        fuel_cell.power_level = power_level as i16;
    }

    Grid {
        grid,
    }
}

pub struct Coordinate(usize, usize);

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[aoc(day11, part1, Chars)]
pub fn part1_chars(grid: &Grid) -> Coordinate {
    let mut largest_power = 0;
    let mut largest_x = 0;
    let mut largest_y = 0;

    for x in 0..298 {
        for y in 0..298 {
            let sub_grid = grid.select_3x3(x, y);
            let total_power = sub_grid.iter().map(|fc| fc.power_level).sum();

            if total_power > largest_power {
                largest_power = total_power;
                largest_x = x + 1;
                largest_y = y + 1;
            }
        }
    }

    Coordinate(largest_x, largest_y)
}

#[derive(Debug, PartialEq)]
pub struct SizedCoordinate(usize, usize, usize);

impl Display for SizedCoordinate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

#[aoc(day11, part2, Chars)]
pub fn part2_chars(grid: &Grid) -> SizedCoordinate {
    let mut largest_power = 0;
    let mut largest_x = 0;
    let mut largest_y = 0;
    let mut largest_size = 0;
    let mut sums_grid = (*grid).clone();

    // Precalculate sums
    for i in 0..=GRID_SIZE {
        if i % 300 == 0 {
            continue;
        }

        let last_fuel_cell_power_level = sums_grid.grid[i - 1].power_level;
        let fuel_cell = &mut sums_grid.grid[i];

        fuel_cell.power_level += last_fuel_cell_power_level;
    }

    for size in 2..=300 {
        let num_sub_grids = 300 - size + 1;

        for x in 0..num_sub_grids {
            for y in 0..num_sub_grids {
                let total_power = sums_grid.sum_sub_grid(x, y, size);

                if total_power > largest_power {
                    largest_power = total_power;
                    largest_x = x + 1;
                    largest_y = y + 1;
                    largest_size = size;
                }
            }
        }
    }

    SizedCoordinate(largest_x, largest_y, largest_size)
}

#[test]
fn test_power_cell_examples() {
    let grid = input_generator("57");

    assert_eq!(grid.grid[121 + 78 * 300].power_level, -5);

    let grid = input_generator("39");

    assert_eq!(grid.grid[216 + 195 * 300].power_level, 0);

    let grid = input_generator("71");

    assert_eq!(grid.grid[100 + 152 * 300].power_level, 4);

    let grid = input_generator("18");

    assert_eq!(grid.select_3x3(32, 44).iter().map(|fc| fc.power_level).sum::<i16>(), 29);
}

#[test]
fn test_power_cell_examples_p2() {
    let grid = input_generator("18");
    let p2 = part2_chars(&grid);

    assert_eq!(p2, SizedCoordinate(90, 269, 16));
}
