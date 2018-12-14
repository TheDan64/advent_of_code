use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, self};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FORMAT: Regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<char, Vec<char>> {
    let mut before_steps = HashMap::new();

    for cap in FORMAT.captures_iter(input) {
        let step = cap[1].chars().next().unwrap();
        let before_step = cap[2].chars().next().unwrap();
        let entry = before_steps.entry(before_step).or_insert_with(Vec::new);

        entry.push(step);
    }

    before_steps
}

static CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn apply_to_options<F>(steps: &HashMap<char, Vec<char>>, mut f: F)
where
    F: FnMut(&[char], &mut HashSet<char>),
{
    let mut executed_steps = HashSet::<char>::new();
    let mut options = Vec::new();

    loop {
        options.clear();

        for ch in CHARS.chars() {
            if !executed_steps.contains(&ch) && !steps.get(&ch).is_some() {
                options.push(ch);
            }

            if !executed_steps.contains(&ch) {
                if let Some(steps) = steps.get(&ch) {
                    if steps.iter().all(|ch2| executed_steps.contains(ch2)) {
                        options.push(ch);
                    }
                }
            }
        }

        options.sort();

        f(&options, &mut executed_steps);

        if executed_steps.len() == CHARS.len() {
            break;
        }
    }
}

#[aoc(day7, part1, Chars)]
pub fn part1_chars(steps: &HashMap<char, Vec<char>>) -> String {
    let mut steps_string = String::new();

    apply_to_options(steps, |options, executed_steps| {
        for ch in options.iter() {
            if !executed_steps.contains(ch) {
                executed_steps.insert(*ch);
                steps_string.push(*ch);
                break;
            }
        }
    });

    steps_string
}

const MAX_WORKERS: usize = 5;

#[derive(Debug)]
pub struct TimeAndSteps(u64, String);

impl Display for TimeAndSteps {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[aoc(day7, part2, Chars)]
pub fn part2_chars(steps: &HashMap<char, Vec<char>>) -> TimeAndSteps {
    let mut steps_string = String::new();
    let mut workers: [Option<(char, u8)>; MAX_WORKERS] = [None; MAX_WORKERS];
    let mut total_seconds = 0;

    apply_to_options(steps, |options, executed_steps| {
        total_seconds += 1;

        for ch in options.iter() {
            let check_opt_char = |opt: &Option<(char, u8)>| {
                if let Some(work) = opt {
                    work.0 == *ch
                } else {
                    false
                }
            };

            if let Some(existing_slot) = workers.iter().position(check_opt_char) {
                let worker = &mut workers[existing_slot];
                let timer = &mut worker.as_mut().unwrap().1;

                *timer -= 1;

                continue;
            }

            if let Some(free_slot) = workers.iter().position(|w| w.is_none()) {
                let timer = *ch as u8 - 5;
                let worker = &mut workers[free_slot];

                *worker = Some((*ch, timer))
            }
        }

        for opt_worker in workers.iter_mut() {
            if let Some(worker) = opt_worker {
                if worker.1 == 0 {
                    executed_steps.insert(worker.0);
                    steps_string.push(worker.0);
                    *opt_worker = None;
                }
            }
        }
    });

    TimeAndSteps(total_seconds, steps_string)
}
