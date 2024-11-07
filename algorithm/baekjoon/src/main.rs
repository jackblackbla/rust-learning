use std::io;

fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();

    println!("{} {}", min, max);
}