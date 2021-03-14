mod day01;
mod day02;
mod puzzle;

fn main() {
    let now = std::time::Instant::now();
    // TODO how to hold a list of "puzzles" in a vector?
    puzzle::run_puzzle(&day01::Day01 {});
    puzzle::run_puzzle(&day02::Day02 {});
    println!("Total time: {} s", now.elapsed().as_secs_f64());
}
