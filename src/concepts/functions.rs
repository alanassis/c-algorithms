fn main() {
    let double: u16 = double(1);
    println!("{double}"); // 2

    let triple: u16 = triple(1);
    println!("{triple}"); // 3
}

fn double(num: u16) -> u16 {
    num * 2
}

fn triple(num: u16) -> u16 {
    return num * 3;
}