use crate::puzzle::{Info, Puzzle};
pub struct Day15 {}

impl Puzzle for Day15 {
    type InputType = Vec<usize>;
    type T1 = usize;
    type T2 = usize;

    fn info(&self) -> Info {
        Info {
            name: "Rambunctious Recitation",
            year: 2020,
            day: 15,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        return vec![6, 4, 12, 1, 20, 0, 16];
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        solve(&input, 2020)
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        solve(&input, 30_000_000)
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (475, 11261)
    }
}

/// This is the Van Eck Sequence: https://www.numberphile.com/videos/van-eck-sequence.
/// There is no known way to compute the nth number without computing the entire
/// sequence up to n. Some optimizations are possible (e.g.
/// https://github.com/timvisee/advent-of-code-2020/blob/master/day15b/src/main.rs)
/// but nothing radical.
fn solve(input: &[usize], limit: usize) -> usize {
    let mut input0 = input.to_vec();
    let mut last = input0.pop().unwrap();
    let mut array = vec![0; limit];

    for turn in 1..=input0.len() {
        array[input0[turn - 1]] = turn;
    }

    for turn in input.len() + 1..=limit {
        let index_of_last = array[last];
        let next = if index_of_last == 0 {
            0
        } else {
            turn - index_of_last - 1
        };
        array[last] = turn - 1;
        last = next;
    }

    last
}
