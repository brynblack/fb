use fb::Fibonacci;
use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let num = buf
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    Fibonacci::new(0f64, 1f64)
        .take(num)
        .for_each(|n| println!("{n}"));
}
