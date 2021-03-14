mod day01;
mod puzzle;

fn main() {
    let now = std::time::Instant::now();
    match day01::parse_input() {
        Ok(input) => {
            let r1 = day01::part1(&input);
            let r2 = day01::part2(&input);
            println!("Time: {} s", now.elapsed().as_secs_f64());

            let actual = (r1.unwrap(), r2.unwrap());
            assert_eq!(actual, day01::expected());
        }
        Err(what) => {
            panic!(what);
        }
    }

    let p = day01::Day01 {};
    puzzle::run_puzzle(&p);
}
