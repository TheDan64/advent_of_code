use std::collections::HashMap;

use chrono::{NaiveDateTime, NaiveDate, Timelike};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FORMAT: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] \w+ (#(\d+))?").unwrap();
}

#[derive(Debug)]
enum UnfinishedRowType {
    Begin { guard_id: u32 },
    Wake,
    FallAsleep,
}

pub enum ActionType {
    Begin,
    Wake,
    FallAsleep,
}

#[derive(Debug)]
struct UnfinishedRow {
    datetime: NaiveDateTime,
    row_type: UnfinishedRowType,
}

pub struct GuardAction {
    datetime: NaiveDateTime,
    guard_id: u32,
    action_type: ActionType,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<GuardAction> {
    let mut unfinished_rows: Vec<_> = FORMAT.captures_iter(input).map(|cap| {
        let year = cap[1].parse::<i32>().unwrap();
        let month = cap[2].parse::<u32>().unwrap();
        let day = cap[3].parse::<u32>().unwrap();
        let hour = cap[4].parse::<u32>().unwrap();
        let minute = cap[5].parse::<u32>().unwrap();
        let guard_num = cap.get(7).map(|m| m.as_str().parse::<u32>().unwrap());

        let row_type = if let Some(guard_id) = guard_num {
            UnfinishedRowType::Begin { guard_id }
        } else if cap[0].ends_with("wakes ") {
            UnfinishedRowType::Wake
        } else {
            UnfinishedRowType::FallAsleep
        };

        UnfinishedRow {
            datetime: NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 0),
            row_type,
        }
    }).collect();

    unfinished_rows.sort_by(|row, row2| row.datetime.cmp(&row2.datetime));

    let mut guard_id = 0;

    unfinished_rows.iter()
        .map(|row| {
            let action_type = match row.row_type {
                UnfinishedRowType::Begin { guard_id: id } => {
                    guard_id = id;

                    ActionType::Begin
                },
                UnfinishedRowType::FallAsleep => ActionType::FallAsleep,
                UnfinishedRowType::Wake => ActionType::Wake,
            };

            GuardAction {
                datetime: row.datetime,
                guard_id,
                action_type,
            }
        })
        .collect()
}

fn get_time_and_minutes_asleep(actions: &[GuardAction]) -> (HashMap<u32, u32>, HashMap<(u32, u32), u32>) {
    let mut time_asleep = HashMap::new();
    let mut minutes_asleep = HashMap::new();
    let mut fell_asleep_at = None;

    for action in actions {
        match action.action_type {
            ActionType::FallAsleep => fell_asleep_at = Some(action.datetime),
            ActionType::Wake => {
                let time_asleep = time_asleep.entry(action.guard_id).or_insert(0);
                let diff = action.datetime - fell_asleep_at.unwrap();

                *time_asleep += diff.num_minutes() as u32;

                let start_minute = fell_asleep_at.unwrap().minute();
                let end_minute = start_minute + diff.num_minutes() as u32;

                for minute in start_minute..end_minute {
                    let minute = minute % 60;
                    let minute_asleep = minutes_asleep.entry((action.guard_id, minute)).or_insert(0);

                    *minute_asleep += 1;
                }

                fell_asleep_at = None;
            },
            _ => {},
        }
    }

    (time_asleep, minutes_asleep)
}

#[aoc(day4, part1, Chars)]
pub fn part1_chars(actions: &[GuardAction]) -> u32 {
    let (time_asleep, minutes_asleep) = get_time_and_minutes_asleep(&actions);
    let mut guard_id = 0;
    let mut max_time_asleep = 0;

    for (id, time_asleep) in time_asleep.iter() {
        if *time_asleep > max_time_asleep {
            max_time_asleep = *time_asleep;
            guard_id = *id;
        }
    }

    let mut most_asleep_minute = 0;
    let mut most_time_asleep = 0;

    let minutes_iter = minutes_asleep.iter()
        .filter(|((id, _), _)| *id == guard_id);

    for ((_, minute), current_time_asleep) in minutes_iter {
        if *current_time_asleep > most_time_asleep {
            most_time_asleep = *current_time_asleep;
            most_asleep_minute = *minute;
        }
    }

    guard_id * most_asleep_minute
}

#[aoc(day4, part2, Chars)]
pub fn part2_chars(actions: &[GuardAction]) -> u32 {
    let (_, minutes_asleep) = get_time_and_minutes_asleep(actions);
    let mut minute = 0;
    let mut guard_id = 0;
    let mut times_asleep = 0;

    for ((id, min), current_times_asleep) in minutes_asleep {
        if current_times_asleep > times_asleep {
            guard_id = id;
            times_asleep = current_times_asleep;
            minute = min;
        }
    }

    guard_id * minute
}
