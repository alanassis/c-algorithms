fn main() {
    const IS_ONE_POSITIVE: bool = if 1 > 0 { true } else { false };
    println!("Is one positive? {}", IS_ONE_POSITIVE);

    let http_response: bool = loop {
        let http_code: u16 = 200;
        if http_code == 200 { break true }
    };
    println!("Http response: {}", http_response);

    let friends = ["Jonatas", "Artur"];
    for friend in friends {
        println!("Friend: {friend}");
    }

    for left in (1..=3).rev() {
        println!("{left} Seconds left...");
    }
}