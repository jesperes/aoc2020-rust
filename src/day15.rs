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
        solve::<2020>(&input)
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        solve::<30000000>(&input)
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (475, 11261)
    }
}

fn solve<const LIMIT: usize>(input: &[usize]) -> usize {
    let mut input0 = input.to_vec();
    let mut last = input0.pop().unwrap();
    let mut array = vec![0; LIMIT];

    for turn in 1..=input0.len() {
        array[input0[turn - 1]] = turn;
    }

    // TODO optimize this inner loop.
    let mut turn = input.len() + 1;
    while turn <= LIMIT {
        let index_of_last = array[last];
        let next = if index_of_last == 0 {
            0
        } else {
            turn - index_of_last - 1
        };
        array[last] = turn - 1;
        turn += 1;
        last = next;
    }

    last
}
