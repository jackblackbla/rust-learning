use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: u16 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b: u16 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c: u16 = input.trim().parse().unwrap();

    let result: u32 = (a as u32) * (b as u32) * (c as u32);

    let mut counts = [0; 10];

    for ch in result.to_string().chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        counts[digit] += 1;
    }

    for count in counts.iter() {
        println!("{}", count);
    }
}