use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    loop {
        math_game();
    }
}

fn math_game() {
    println!("Hi! Can you resolve some maths?");

    let n1: u8 = rand::thread_rng().gen_range(1..=100);
    let n2: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Question: {n1} + {n2}?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something wrong happened :(");

    let input: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => return,
    };

    let sum: u8 = n1 + n2;

    match sum.cmp(&input) {
        Ordering::Equal => println!("Correct!"),
        _ => println!("Incorrect!"),
    }
}
