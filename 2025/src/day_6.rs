// https://adventofcode.com/2025/day/6

static FILE_INPUT: &'static str = include_str!("../input_6.txt");
// static FILE_INPUT: &'static str = include_str!("../input_6_exemple.txt");

pub fn solve() {
    let lines: Vec<&str> = FILE_INPUT.lines().collect();

    // Pad all lines to same length
    let max_width = lines.iter().map(|l| l.len()).max().unwrap();
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_width))
        .collect();

    let operator_line = &padded_lines[padded_lines.len() - 1];
    let number_lines = &padded_lines[..padded_lines.len() - 1];

    let mut grand_total = 0i64;
    let mut processed = vec![false; max_width];
    let mut col = max_width;

    while col > 0 {
        col -= 1;

        if processed[col] {
            continue;
        }

        let op = operator_line.chars().nth(col).unwrap();
        if op == ' ' {
            continue;
        }

        // Found an operator
        let mut left = col;
        let mut right = col;

        // Expand left to find all columns that are part of this problem
        while left > 0 {
            let check_col = left - 1;
            let mut is_separator = true;
            for line in padded_lines.iter() {
                let ch = line.chars().nth(check_col).unwrap();
                if ch != ' ' {
                    is_separator = false;
                    break;
                }
            }
            if is_separator {
                break;
            }
            left -= 1;
        }

        // Expand right to find all columns that are part of this operation
        while right < max_width - 1 {
            let check_col = right + 1;
            let mut is_separator = true;
            for line in padded_lines.iter() {
                let ch = line.chars().nth(check_col).unwrap();
                if ch != ' ' {
                    is_separator = false;
                    break;
                }
            }
            if is_separator {
                break;
            }
            right += 1;
        }

        // collect all numbers from right to left
        let mut numbers = Vec::new();
        for c in (left..=right).rev() {
            processed[c] = true;

            let mut num_str = String::new();
            for line in number_lines.iter() {
                let ch = line.chars().nth(c).unwrap();
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                }
            }

            if !num_str.is_empty() {
                numbers.push(num_str.parse::<i64>().unwrap());
            }
        }

        let result = if !numbers.is_empty() {
            match op {
                '+' => numbers.iter().sum(),
                '*' => numbers.iter().product(),
                _ => 0,
            }
        } else {
            0
        };

        log::info!(
            "Operation at col {} (range {}..={}): {:?} {} = {}",
            col,
            left,
            right,
            numbers,
            op,
            result
        );
        grand_total += result;
    }

    log::info!("Total: {}", grand_total);
}

#[allow(dead_code)]
pub fn solve_part_1() {
    let mut operations: Vec<Vec<String>> = Vec::new();

    let mut total = 0;

    for line in FILE_INPUT.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts = line.split(' ');
        log::debug!("Analysing line index {line}");
        let mut operation_index = 0;
        let mut operation_part_index = 0;

        for part in parts {
            if part.is_empty() || part == " " {
                // log::debug!("Skipping empty part at index {index}");
                continue;
            }

            if part == "+" || part == "*" {
                log::debug!("Make operation {:?}", operations);
                let o = operations.get(operation_part_index).unwrap();
                // for o in operations.clone() {
                let mut subtotal = 0;
                for ope_part in o {
                    let n = ope_part.parse::<i128>().unwrap();
                    if part == "+" {
                        subtotal += n;
                    } else if part == "*" {
                        if subtotal > 0 {
                            subtotal *= n;
                        } else {
                            subtotal = n;
                        }
                    }
                }
                log::debug!("{part}, {subtotal}, {:?}", o);
                total += subtotal;
                operation_part_index += 1;
                // }
            } else {
                if operations.get(operation_index).is_none() {
                    let v = vec![part.to_string()];
                    operations.insert(operation_index, v.clone());
                    log::debug!("Create index {operation_index} with {:?}", v);
                } else {
                    operations[operation_index].push(part.to_string());
                    log::debug!("Store to index {operation_index} {part}");
                }
            }
            operation_index += 1;
        }
    }
    log::info!("Total count: {total}");
}
