use fb::Fibonacci;
use std::env::args;

fn main() {
    Fibonacci::new(0f64, 1f64)
        .take(args().nth(1).unwrap().parse().unwrap())
        .for_each(|n| println!("{n}"));
}
