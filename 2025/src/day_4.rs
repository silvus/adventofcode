// https://adventofcode.com/2025/day/4

static FILE_INPUT: &'static str = include_str!("../input_4.txt");
// static FILE_INPUT: &'static str = include_str!("../input_4_exemple.txt");

type Grid = Vec<Vec<char>>;

pub fn grid_remove(grid: Grid) -> (Grid, usize) {
    let h = grid.len();
    let w = grid[0].len();

    let mut grid_removed = grid.clone();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut counts = vec![vec![0usize; w]; h];

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != '@' {
                continue;
            }
            let mut c = 0;
            for (dy, dx) in dirs {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                    if grid[ny as usize][nx as usize] == '@' {
                        c += 1;
                    }
                }
            }
            counts[y][x] = c;
        }
    }

    let mut rolls_accessed = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '@' {
                print!("{}", counts[y][x]);

                if counts[y][x] < 4 {
                    rolls_accessed += 1;

                    // Remove picked roll
                    grid_removed[y][x] = '.';
                }
            } else {
                print!(".");
            }
        }
        println!();
    }

    (grid_removed, rolls_accessed)
}

pub fn solve() {
    let file_vec: Vec<&str> = FILE_INPUT.lines().collect();

    let grid: Vec<Vec<char>> = file_vec.iter().map(|r| r.chars().collect()).collect();

    let mut rolls_accessed_total = 0;

    let mut g = grid.clone();
    loop {
        let (g_new, t) = grid_remove(g.clone());
        rolls_accessed_total += t;
        g = g_new;

        log::info!("Paper rolls: {rolls_accessed_total}");
        if t <= 0 {
            break;
        }
    }
}
