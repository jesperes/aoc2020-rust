use crate::puzzle::{Info, Puzzle};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

pub struct Day17 {}

type Coord = (i32, i32, i32, i32);

#[derive(Debug, Clone)]
pub struct Grid {
    active_cells: HashSet<Coord>, // x, y, z, w
    limits: Limits,
}

#[derive(Debug, Clone, Copy)]
pub struct Limit {
    min: i32,
    max: i32,
}

impl Limit {
    fn new() -> Limit {
        Limit { min: 0, max: 0 }
    }
    fn maybe_widen(&mut self, val: i32) {
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }
    fn to_range(self) -> RangeInclusive<i32> {
        (self.min - 1)..=(self.max + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Limits {
    x: Limit,
    y: Limit,
    z: Limit,
    w: Limit,
}

impl Limits {
    fn new() -> Limits {
        Limits {
            x: Limit::new(),
            y: Limit::new(),
            z: Limit::new(),
            w: Limit::new(),
        }
    }
}

impl Puzzle for Day17 {
    type InputType = Grid;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Conway Cubes",
            year: 2020,
            day: 17,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let mut input = Grid {
            active_cells: HashSet::new(),
            limits: Limits::new(),
        };
        let info = self.info();
        let mut y = 0;

        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .for_each(|line| {
            let mut x = 0;
            line.unwrap().chars().for_each(|c| {
                if c == '#' {
                    input.active_cells.insert((x, y, 0, 0));
                    input.limits.x.maybe_widen(x);
                    input.limits.y.maybe_widen(y);
                }
                x += 1;
            });
            y += 1;
        });

        input
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut input_mut = input.clone();

        for _ in 0..6 {
            let mut next_gen_active_cells: HashSet<Coord> = HashSet::new();

            for x in &mut input_mut.limits.x.to_range() {
                for y in &mut input_mut.limits.y.to_range() {
                    for z in &mut input_mut.limits.z.to_range() {
                        let mut active_nbrs = 0;

                        // Count active neighbors
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                for dz in -1..=1 {
                                    if dx == 0 && dy == 0 && dz == 0 {
                                        continue;
                                    }
                                    if input_mut
                                        .active_cells
                                        .contains(&(x + dx, y + dy, z + dz, 0))
                                    {
                                        active_nbrs += 1;
                                    }
                                }
                            }
                        }

                        let cell = (x, y, z, 0);
                        let is_active = input_mut.active_cells.contains(&cell);

                        if (is_active && active_nbrs >= 2 && active_nbrs <= 3)
                            || (!is_active && active_nbrs == 3)
                        {
                            next_gen_active_cells.insert(cell);
                            input_mut.limits.x.maybe_widen(x);
                            input_mut.limits.y.maybe_widen(y);
                            input_mut.limits.z.maybe_widen(z);
                        }
                    }
                }
            }

            input_mut.active_cells = next_gen_active_cells;
        }

        input_mut.active_cells.len() as i32
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut input_mut = input.clone();

        for _ in 0..6 {
            let mut next_gen_active_cells: HashSet<Coord> = HashSet::new();
            let mut next_gen_limits = Limits::new();

            for x in &mut input_mut.limits.x.to_range() {
                for y in &mut input_mut.limits.y.to_range() {
                    for z in &mut input_mut.limits.z.to_range() {
                        for w in &mut input_mut.limits.w.to_range() {
                            let mut active_nbrs = 0;

                            // Count active neighbors
                            for dx in -1..=1 {
                                for dy in -1..=1 {
                                    for dz in -1..=1 {
                                        for dw in -1..=1 {
                                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                                continue;
                                            }
                                            if input_mut.active_cells.contains(&(
                                                x + dx,
                                                y + dy,
                                                z + dz,
                                                w + dw,
                                            )) {
                                                active_nbrs += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            let cell = (x, y, z, w);
                            let is_active = input_mut.active_cells.contains(&cell);

                            if (is_active && active_nbrs >= 2 && active_nbrs <= 3)
                                || (!is_active && active_nbrs == 3)
                            {
                                next_gen_active_cells.insert(cell);
                                next_gen_limits.x.maybe_widen(x);
                                next_gen_limits.y.maybe_widen(y);
                                next_gen_limits.z.maybe_widen(z);
                                next_gen_limits.w.maybe_widen(w);
                            }
                        }
                    }
                }
            }

            input_mut.active_cells = next_gen_active_cells;
            input_mut.limits = next_gen_limits;
        }

        input_mut.active_cells.len() as i32
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (242, 2292)
    }
}
