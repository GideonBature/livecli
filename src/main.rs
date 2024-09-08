// A Simple CLI in Rust
// This CLI will get a string from the input
// and return the string reversed
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a string");
        return;
    }

    let input: String = args[1].clone();

    let reversed = input.chars().rev().collect::<String>();

    println!("Reversed string: {}", reversed);
}
