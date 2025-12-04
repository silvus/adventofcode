// https://adventofcode.com/2025/day/3

static FILE_INPUT: &'static str = include_str!("../input_3.txt");
// static FILE_INPUT: &'static str = include_str!("../input_3_exemple.txt");

pub fn solve() {
    let mut joltage_max = 0;
    for line in FILE_INPUT.lines() {
        let k = 12;
        let mut stack = Vec::with_capacity(k);
        let mut to_remove = line.len() - k;

        for c in line.chars() {
            while let Some(&last) = stack.last() {
                if to_remove > 0 && last < c {
                    stack.pop();
                    to_remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(c);
        }

        let bat: String = stack.into_iter().take(k).collect();

        log::debug!("{bat}");
        joltage_max += bat.parse::<i64>().unwrap();
    }
    log::info!("Joltage max {joltage_max}")
}

#[allow(dead_code)]
pub fn solve_part_1() {
    let file_lines_count = FILE_INPUT.lines().count();

    let mut joltage_max = 0;

    log::info!("Iterating on {file_lines_count} batteries");

    for line in FILE_INPUT.lines() {
        let mut biggest_number_first = 0;
        let mut biggest_number_last = 0;
        let line_len = line.trim().chars().count();
        let mut first_number_index = 0;
        let mut last_number_index = 0;

        let mut biggest_number_first_is_last = false;

        for (i, c) in line.trim().chars().enumerate() {
            let c_int = c.to_string().parse::<i32>().unwrap();
            // biggest_number_first = biggest_number_first.max(c_int);
            if c_int > biggest_number_first {
                biggest_number_first = c_int;
                first_number_index = i;
            }
        }

        log::debug!("First big number {biggest_number_first}, index {first_number_index}");
        let mut line_analyse = String::from(line);
        line_analyse.insert(first_number_index, '_');
        line_analyse.insert(first_number_index + 2, '_');
        log::debug!("{:?}", line_analyse);

        for (i, c) in line.trim().chars().enumerate() {
            // Special case if first biggest number is on last positon
            if first_number_index + 1 == line_len {
                biggest_number_first_is_last = true;
                let c_int = c.to_string().parse::<i32>().unwrap();
                if c_int > biggest_number_last && i < first_number_index {
                    biggest_number_last = c_int;
                    last_number_index = i;
                }
            } else {
                if i > first_number_index {
                    let c_int = c.to_string().parse::<i32>().unwrap();
                    if c_int > biggest_number_last {
                        biggest_number_last = c_int;
                        last_number_index = i;
                    }
                }
            }
        }
        log::debug!("Second big number {biggest_number_last}, index {last_number_index}");
        let mut line_analyse = String::from(line);
        line_analyse.insert(last_number_index, '_');
        line_analyse.insert(last_number_index + 2, '_');
        log::debug!("{:?}", line_analyse);

        let biggest_number = if biggest_number_first_is_last {
            format!("{biggest_number_last}{biggest_number_first}")
        } else {
            format!("{biggest_number_first}{biggest_number_last}")
        };

        log::debug!("Biggest number: {biggest_number}");

        joltage_max += biggest_number.parse::<i32>().unwrap();
    }
    log::info!("Joltage max {joltage_max}")
}
