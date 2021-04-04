use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day11 {}

impl Puzzle for Day11 {
    type InputType = Vec<Vec<char>>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Seating System",
            year: 2020,
            day: 11,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut grid_a = input.to_vec();
        let mut grid_b = input.to_vec();
        let mut round = 0;

        loop {
            let result = if round % 2 == 0 {
                do_round(&grid_a, &mut grid_b)
            } else {
                do_round(&grid_b, &mut grid_a)
            };
            round += 1;
            match result {
                Some(count) => return count,
                None => continue,
            }
        }
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut grid_a = input.to_vec();
        let mut grid_b = input.to_vec();
        let mut round = 0;

        loop {
            let result = if round % 2 == 0 {
                do_round2(&grid_a, &mut grid_b)
            } else {
                do_round2(&grid_b, &mut grid_a)
            };
            round += 1;
            match result {
                Some(count) => return count,
                None => continue,
            }
        }
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (2093, 1862)
    }
}

fn do_round(read_grid: &[Vec<char>], write_grid: &mut [Vec<char>]) -> Option<i32> {
    let mut changed = false;
    let mut occupied_seats = 0;
    let height = read_grid.len();
    let width = read_grid[0].len();

    for y in 0..height {
        for x in 0..width {
            let adj = number_occupied_adjacents(x as i32, y as i32, &read_grid);
            let c = read_grid[y][x];
            let next_c = match c {
                'L' if adj == 0 => '#',
                '#' if adj >= 4 => 'L',
                other => other,
            };
            if next_c != c {
                changed = true;
            }
            if next_c == '#' {
                occupied_seats += 1;
            }
            write_grid[y][x] = next_c;
        }
    }
    if !changed {
        Some(occupied_seats)
    } else {
        None
    }
}

fn do_round2(read_grid: &[Vec<char>], write_grid: &mut [Vec<char>]) -> Option<i32> {
    let mut changed = false;
    let mut occupied_seats = 0;
    let height = read_grid.len();
    let width = read_grid[0].len();

    for y in 0..height {
        for x in 0..width {
            let adj = number_occupied_adjacents2(x as i32, y as i32, &read_grid);
            let c = read_grid[y][x];
            let next_c = match c {
                'L' if adj == 0 => '#',
                '#' if adj >= 5 => 'L',
                other => other,
            };
            if next_c != c {
                changed = true;
            }
            if next_c == '#' {
                occupied_seats += 1;
            }
            write_grid[y][x] = next_c;
        }
    }
    if !changed {
        Some(occupied_seats)
    } else {
        None
    }
}

#[inline]
fn read_grid(grid: &[Vec<char>], x: i32, y: i32) -> Option<&char> {
    if let Some(row) = grid.get(y as usize) {
        row.get(x as usize)
    } else {
        None
    }
}

fn number_occupied_adjacents(x: i32, y: i32, grid: &[Vec<char>]) -> i32 {
    let mut count = 0;
    lazy_static! {
        static ref DELTAS: Vec<(i32, i32)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
    }
    for (dx, dy) in DELTAS.iter() {
        if let Some(c) = read_grid(grid, x + dx, y + dy) {
            if *c == '#' {
                count += 1;
            }
        }
    }
    count
}

fn number_occupied_adjacents2(x: i32, y: i32, grid: &[Vec<char>]) -> i32 {
    let mut count = 0;
    lazy_static! {
        static ref DELTAS: Vec<(i32, i32)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
    }
    for (dx, dy) in DELTAS.iter() {
        let mut dist = 1;
        while let Some(c) = read_grid(grid, x + dx * dist, y + dy * dist) {
            match c {
                '#' => {
                    count += 1;
                    break;
                }
                'L' => break,
                '.' => dist += 1,
                _ => break,
            }
        }
    }
    count
}
