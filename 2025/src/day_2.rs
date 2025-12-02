// https://adventofcode.com/2025/day/2

static FILE_INPUT: &'static str = include_str!("../input_2.txt");
// static FILE_INPUT: &'static str = include_str!("../input_2_exemple.txt");

// fn has_repeated_pattern(s: &str) -> bool {
//     let len = s.len();

//     for i in 0..len {
//         for pattern_len in (2..=(len - i)).step_by(2) {
//             if i + pattern_len > len {
//                 break;
//             }

//             let mid = i + pattern_len / 2;
//             if mid + pattern_len / 2 <= len {
//                 let first_half = &s[i..mid];
//                 let second_half = &s[mid..mid + pattern_len / 2];

//                 if first_half == second_half {
//                     return true;
//                 }
//             }
//         }
//     }

//     false
// }

fn is_fully_repeated(s: &str) -> bool {
    let len = s.len();

    // Must be even length to be split into two equal parts
    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    &s[..mid] == &s[mid..]
}

fn is_fully_repeated_v2(s: &str) -> bool {
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Check if the length is divisible by pattern_len
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];

            // Check if the whole string is made of this pattern repeated
            let mut is_repeated = true;
            for i in (pattern_len..len).step_by(pattern_len) {
                if &s[i..i + pattern_len] != pattern {
                    is_repeated = false;
                    break;
                }
            }

            if is_repeated {
                return true;
            }
        }
    }

    false
}

pub fn solve() {
    let id_list = FILE_INPUT.split(",");

    // println!("{}", has_repeated_pattern("5454412312336789")); // true
    // println!("{}", has_repeated_pattern("55")); // true
    // println!("{}", has_repeated_pattern("123123")); // true
    // println!("{}", has_repeated_pattern("1234")); // false
    // println!("{}", has_repeated_pattern("abcdef")); // false

    // println!("{}", is_fully_repeated("38593859")); // true
    // println!("{}", is_fully_repeated("38593850")); // false
    // println!("{}", is_fully_repeated("55")); // true
    // println!("{}", is_fully_repeated("6464")); // true
    // println!("{}", is_fully_repeated("123123")); // true
    // println!("{}", is_fully_repeated("1234")); // false
    // println!("{}", is_fully_repeated("121")); // false

    // Part 2
    // println!("{}", is_fully_repeated_v2("12341234")); // true (1234 x2)
    // println!("{}", is_fully_repeated_v2("123123123")); // true (123 x3)
    // println!("{}", is_fully_repeated_v2("1212121212")); // true (12 x5)
    // println!("{}", is_fully_repeated_v2("1111111")); // true (1 x7)
    // println!("{}", is_fully_repeated_v2("12345678")); // false (no repetition)
    // println!("{}", is_fully_repeated_v2("123456")); // false (no repetition)

    let mut global_count_part_1 = 0;
    let mut global_count_part_2 = 0;

    for id_range in id_list {
        let mut id_split = id_range.split("-");

        let id_first = id_split.next().unwrap().trim();
        let id_last = id_split.next().unwrap().trim();

        log::info!("Analyse IDs between {:?} and {:?}", id_first, id_last);

        let range_start = id_first.parse::<i64>().unwrap();
        let range_stop = id_last.parse::<i64>().unwrap() + 1;

        for r in range_start..range_stop {
            let is_repeated_part_1 = is_fully_repeated(&r.to_string());
            if is_repeated_part_1 {
                log::debug!("Repeating part 1 {}", r);
                global_count_part_1 += r;
            }

            let is_repeated_part_2 = is_fully_repeated_v2(&r.to_string());
            if is_repeated_part_2 {
                log::debug!("Repeating part 2 {}", r);
                global_count_part_2 += r;
            }
        }
    }

    log::info!("Total part 1: {global_count_part_1}");
    log::info!("Total part 2: {global_count_part_2}");
}
