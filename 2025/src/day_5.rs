// https://adventofcode.com/2025/day/5

use std::ops::RangeInclusive;

static FILE_INPUT: &'static str = include_str!("../input_5.txt");
// static FILE_INPUT: &'static str = include_str!("../input_5_exemple.txt");

fn merge_intervals(mut intervals: Vec<RangeInclusive<i128>>) -> Vec<RangeInclusive<i128>> {
    intervals.sort_by_key(|r| *r.start());
    let mut merged: Vec<RangeInclusive<i128>> = Vec::new();

    for r in intervals {
        if let Some(last) = merged.last_mut() {
            if r.start() <= last.end() {
                let start = *last.start();
                let end = (*last.end()).max(*r.end());
                *last = start..=end;
                continue;
            }
        }
        merged.push(r);
    }

    merged
}

fn interval_contains(intervals: &[RangeInclusive<i128>], value: i128) -> bool {
    intervals
        .binary_search_by(|r| {
            if value < *r.start() {
                std::cmp::Ordering::Greater
            } else if value > *r.end() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

#[allow(dead_code)]
pub fn solve_part_1() {
    let file_lines = FILE_INPUT.lines();

    let mut intervals: Vec<Vec<RangeInclusive<i128>>> = Vec::new();
    let mut ids: Vec<i128> = Vec::new();

    for line in file_lines {
        if line.trim().is_empty() {
            continue;
        }

        let mut l = line.split('-');
        let count = l.clone().count();
        if count >= 2 {
            // If there are 2 numbers, keep them has range
            let number_left = l.next().unwrap().parse::<i128>().unwrap();
            let number_right = l.next().unwrap().parse::<i128>().unwrap();
            intervals.push(merge_intervals(vec![number_left..=number_right]));
        } else {
            // If there is only one number, keep it has id
            let number_left = l.next().unwrap().parse::<i128>().unwrap();
            // log::debug!("Keep ID {number_left}");
            ids.push(number_left);
        }
    }

    let mut total_fresh_ingredients = 0;

    for id in ids {
        for interval in intervals.clone() {
            let id_is_in_range = interval_contains(&interval, id);
            if id_is_in_range {
                log::debug!("ID {id} is in range {:?}", interval);
                total_fresh_ingredients += 1;
                break;
            }
        }
    }

    log::info!("Total fresh ingrediens: {total_fresh_ingredients}");
}

pub fn solve() {
    let mut ranges = Vec::new();

    for line in FILE_INPUT.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split('-');
        if parts.clone().count() == 2 {
            let a = parts.next().unwrap().parse::<i128>().unwrap();
            let b = parts.next().unwrap().parse::<i128>().unwrap();
            ranges.push(a..=b);
        }
    }

    let merged = merge_intervals(ranges);

    // Range 3 to 5 give integers 3, 4, 5
    // end - start = 5 - 3 = 2
    // Add 1 to get 3 integers
    let total_count: i128 = merged.iter().map(|r| r.end() - r.start() + 1).sum();

    log::info!("Total count of IDs in ranges: {total_count}");
}
