use aoc_runner_derive::aoc;

fn is_visible(map: &[&str], coord: (usize, usize), n_rows: usize, n_columns: usize) -> bool {
    let (x, y) = coord;
    let val = map[y].as_bytes()[x];

    if x == 0 || y == 0 || x == n_columns - 1 || y == n_rows - 1 {
        return true;
    }

    if (0..x)
        .into_iter()
        .all(|new_x| map[y].as_bytes()[new_x] < val)
    {
        return true;
    }

    if (0..y)
        .into_iter()
        .all(|new_y| map[new_y].as_bytes()[x] < val)
    {
        return true;
    }

    if (x + 1..n_columns)
        .into_iter()
        .all(|new_x| map[y].as_bytes()[new_x] < val)
    {
        return true;
    }

    if (y + 1..n_rows)
        .into_iter()
        .all(|new_y| map[new_y].as_bytes()[x] < val)
    {
        return true;
    }

    false
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let rows: Vec<_> = input.split('\n').collect();
    let n_rows = rows.len();
    let n_columns = rows[0].len();
    let mut visible = 0;

    for x in 0..n_rows {
        for y in 0..n_columns {
            if is_visible(&rows, (x, y), n_rows, n_columns) {
                visible += 1;
            }
        }
    }

    visible
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TakeWhileInclusive<'a, I: 'a, F> {
    iter: &'a mut I,
    predicate: F,
    done: bool,
}

impl<'a, I, F> TakeWhileInclusive<'a, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    /// Create a new [`TakeWhileInclusive`] from an iterator and a predicate.
    pub fn new(iter: &'a mut I, predicate: F) -> Self {
        Self {
            iter,
            predicate,
            done: false,
        }
    }
}

impl<'a, I, F> Iterator for TakeWhileInclusive<'a, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.iter.next().map(|item| {
                if !(self.predicate)(&item) {
                    self.done = true;
                }
                item
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            (0, self.iter.size_hint().1)
        }
    }
}

fn score(map: &[&str], coord: (usize, usize), n_rows: usize, n_columns: usize) -> usize {
    let (x, y) = coord;
    let val = map[y].as_bytes()[x];

    if x == 0 || y == 0 || x == n_columns - 1 || y == n_rows - 1 {
        return 0;
    }

    let mut score = 1;

    let count = TakeWhileInclusive::new(&mut (0..x).into_iter().rev(), |&new_x| {
        map[y].as_bytes()[new_x] < val
    })
    .count();

    score *= count;

    let count = TakeWhileInclusive::new(&mut (0..y).into_iter().rev(), |&new_y| {
        map[new_y].as_bytes()[x] < val
    })
    .count();

    score *= count;

    let count = TakeWhileInclusive::new(&mut (x + 1..n_columns).into_iter(), |&new_x| {
        map[y].as_bytes()[new_x] < val
    })
    .count();

    score *= count;

    let count = TakeWhileInclusive::new(&mut (y + 1..n_rows).into_iter(), |&new_y| {
        map[new_y].as_bytes()[x] < val
    })
    .count();

    score *= count;
    score
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let rows: Vec<_> = input.split('\n').collect();
    let n_rows = rows.len();
    let n_columns = rows[0].len();
    let mut high_score = 0;

    for x in 0..n_rows {
        for y in 0..n_columns {
            let score = score(&rows, (x, y), n_rows, n_columns);

            if score > high_score {
                high_score = score;
            }
        }
    }

    high_score
}
