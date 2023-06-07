fn main() {
    // Scalar types
    let positive: u32 = 666;
    println!("{positive}");

    let negative: i32 = -666;
    println!("{negative}");

    let floating: f64 = 66.6;
    println!("{floating}");

    let boolean: bool = false;
    println!("{boolean}");

    let character: char = 'A';
    println!("{character}");

    // Compound types
    let tuple: (u32, f64) = (666, 66.6);
    println!("{}", tuple.0); // 666

    let array: [u32; 3] = [6, 6, 6];
    println!("{}", array[0]); // 6

    let defined_array: [u32; 3] = [6; 3]; // [6, 6, 6]
    println!("{}", defined_array[0]); // 6
}
