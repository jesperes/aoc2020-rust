use crate::puzzle::{Info, Puzzle};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day14 {}

#[derive(Debug)]
pub enum Instr {
    Mask(HashMap<i32, i32>),
    Mem(i64, i64),
}

impl Puzzle for Day14 {
    type InputType = Vec<Instr>;
    type T1 = i64;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Docking Data",
            year: 2020,
            day: 14,
        }
    }

    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| {
            let s = line.unwrap();
            if let Some(mask) = parse_mask(&s) {
                Instr::Mask(mask)
            } else if let Some((addr, val)) = parse_mem(&s) {
                Instr::Mem(addr, val)
            } else {
                panic!();
            }
        })
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut mem = HashMap::new();
        let default_mask = HashMap::new();
        let mut curr_mask: &HashMap<i32, i32> = &default_mask;
        for instr in input {
            match instr {
                Instr::Mask(mask) => curr_mask = mask,
                Instr::Mem(addr, value) => write_mem(*addr, *value, &curr_mask, &mut mem),
            }
        }
        mem.values().sum()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut mem: HashMap<i64, i64> = HashMap::new();
        let default_mask = HashMap::new();
        let mut curr_mask: &HashMap<i32, i32> = &default_mask;
        let mut num_floating_bits = 0; // number of floating bits in the current mask
        for instr in input {
            match instr {
                Instr::Mask(mask) => {
                    curr_mask = mask;
                    num_floating_bits = curr_mask.values().filter(|v| **v == -1).count();
                }
                Instr::Mem(addr, value) => {
                    // iterate from 0 -> num_floating_bits to span all possible values
                    // of the floating bits.
                    for floating_bit_values in 0..(1 << num_floating_bits) {
                        let mut effective_addr: i64 = *addr;
                        let mut float_bit_idx = 0; // current bit pos in floating_bit_values

                        for (bit_idx, mask_val) in curr_mask.iter() {
                            match mask_val {
                                0 => continue,
                                1 => effective_addr |= 1 << bit_idx,
                                _ => {
                                    if (1 << float_bit_idx) & floating_bit_values != 0 {
                                        effective_addr |= 1 << bit_idx;
                                    } else {
                                        effective_addr &= !(1 << bit_idx);
                                    }
                                    float_bit_idx += 1;
                                }
                            }
                        }
                        mem.insert(effective_addr, *value);
                    }
                }
            }
        }
        mem.values().sum()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (8471403462063, 2667858637669)
    }
}

pub fn write_mem(addr: i64, value: i64, mask: &HashMap<i32, i32>, mem: &mut HashMap<i64, i64>) {
    let mut v = value;
    for (bit, x) in mask {
        if *x == -1 {
            continue;
        }

        if *x == 0 {
            v &= !(1 << bit);
        } else {
            v |= 1 << bit;
        }
    }
    mem.insert(addr, v);
}

pub fn parse_mask(line: &str) -> Option<HashMap<i32, i32>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^mask = ([X01]+)$").unwrap();
    }

    RE.captures(line).map(|cap| {
        let mut map = HashMap::new();
        for (i, c) in cap.get(1).unwrap().as_str().chars().enumerate() {
            let key = 35 - i as i32;

            if c == 'X' {
                map.insert(key, -1);
            } else {
                map.insert(key, (c as i32) - ('0' as i32));
            }
        }
        map
    })
}

pub fn parse_mem(line: &str) -> Option<(i64, i64)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^mem\\[(\\d+)\\] = (\\d+)$").unwrap();
    }

    RE.captures(line).map(|cap| {
        (
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::day14::*;
    #[test]
    fn ex1() {
        let mask = parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        let mem1 = parse_mem("mem[8] = 11").unwrap();
        let mem2 = parse_mem("mem[7] = 101").unwrap();
        let mem3 = parse_mem("mem[8] = 0").unwrap();

        let mut mem = HashMap::new();
        {
            let (addr, val) = mem1;
            write_mem(addr, val, &mask, &mut mem);
            println!("{:?}", mem);
        }
        {
            let (addr, val) = mem2;
            write_mem(addr, val, &mask, &mut mem);
            println!("{:?}", mem);
        }
        {
            let (addr, val) = mem3;
            write_mem(addr, val, &mask, &mut mem);
            println!("{:?}", mem);
        }
    }
}
