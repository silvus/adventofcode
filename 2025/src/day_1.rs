// https://adventofcode.com/2025/day/1

static FILE_INPUT: &'static str = include_str!("../input_1.txt");
static STARTING_POINT: i32 = 50;

pub fn solve() {
    let file_lines_count = FILE_INPUT.lines().count();

    let mut dial = STARTING_POINT;
    let mut password = 0;

    log::info!("Starting at {dial} - Iterating on {file_lines_count} instructions");

    for line in FILE_INPUT.lines() {
        let direction = line.chars().next().unwrap();
        let movements: i32 = line[1..].parse().unwrap();

        for _ in 0..movements {
            if direction == 'L' {
                dial -= 1;
            } else if direction == 'R' {
                dial += 1;
            }

            if dial > 99 {
                dial = 0;
            } else if dial < 0 {
                dial = 99;
            }

            // For second question, all 0 passed are counted
            if dial == 0 {
                password += 1;
            }
        }

        log::debug!("The dial is rotated {direction} {movements} to point at {dial}");
    }
    log::info!("Password: {password}")
}
