pub trait Puzzle {
    fn parse_input(&self) -> Result<i32, &'static str>;
}

pub fn run_puzzle<T: Puzzle>(p: &T) {
    let input = p.parse_input();
    println!("foo{:?}", input);
}
