use std::io;

const GENERIC_ERROR: &str = "Ops, something wrong happened!...";

fn main() {
    let mut fahrenheit = String::new();

    println!("Write a temperature in fahrenheit:");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect(GENERIC_ERROR);

    let fahrenheit: i8 = fahrenheit.trim()
        .parse().expect(GENERIC_ERROR);

    let celsius = fahrenheit_to_celsius(fahrenheit);
    
    println!("{fahrenheit} ºF = {celsius} ºC");
}

fn fahrenheit_to_celsius(f: i8) -> i8 {
    (f - 32) * 5/9
}