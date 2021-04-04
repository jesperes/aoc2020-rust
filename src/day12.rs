use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day12 {}

#[derive(Debug)]
pub enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Puzzle for Day12 {
    type InputType = Vec<Action>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Rain Risk",
            year: 2020,
            day: 12,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| {
            let s = line.unwrap().to_string();
            let num = s[1..].parse::<i32>().unwrap();
            match &s[0..1] {
                "N" => Action::N(num),
                "S" => Action::S(num),
                "E" => Action::E(num),
                "W" => Action::W(num),
                "L" if num % 90 == 0 => Action::L(num),
                "R" if num % 90 == 0 => Action::R(num),
                "F" => Action::F(num),
                _ => panic!(),
            }
        })
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 90; // east (north is 0)

        for instr in input {
            match instr {
                Action::N(num) => y -= num,
                Action::E(num) => x += num,
                Action::S(num) => y += num,
                Action::W(num) => x -= num,
                Action::L(num) => dir = ((dir + 360) - num) % 360,
                Action::R(num) => dir = ((dir + 360) + num) % 360,
                Action::F(num) => match dir {
                    0 => y -= num,
                    90 => x += num,
                    180 => y += num,
                    270 => x -= num,
                    other => panic!("{}", other),
                },
            }
        }
        x.abs() + y.abs()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut x = 0;
        let mut y = 0;
        let mut wp_x = 10;
        let mut wp_y = -1;

        for instr in input {
            match instr {
                Action::N(num) => wp_y -= num,
                Action::E(num) => wp_x += num,
                Action::S(num) => wp_y += num,
                Action::W(num) => wp_x -= num,
                Action::L(num) => rot_wp_left(&mut wp_x, &mut wp_y, *num),
                Action::R(num) => rot_wp_right(&mut wp_x, &mut wp_y, *num),
                Action::F(num) => {
                    x += wp_x * num;
                    y += wp_y * num;
                }
            }
        }
        x.abs() + y.abs()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (845, 27016)
    }
}

fn rot_wp_left(wp_x: &mut i32, wp_y: &mut i32, val: i32) {
    let mut v = val;
    while v > 0 {
        let tmp_x: i32 = *wp_x;
        let tmp_y: i32 = *wp_y;
        *wp_x = tmp_y;
        *wp_y = -tmp_x;
        v -= 90;
    }
}
fn rot_wp_right(wp_x: &mut i32, wp_y: &mut i32, val: i32) {
    let mut v = val;
    while v > 0 {
        let tmp_x: i32 = *wp_x;
        let tmp_y: i32 = *wp_y;
        *wp_x = -tmp_y;
        *wp_y = tmp_x;
        v -= 90;
    }
}
