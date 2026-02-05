use std::io;

fn main() {
    println!("Covert C -> F. Type celcium degrees:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erorr in inpue");

    let input: u32 = input.trim().parse().expect("Type correct value");

    println!(
        "So, {} by celcium is equal to {} by Farengeight",
        input,
        convert(input),
    )
}

fn convert(c_degrees: u32) -> u32 {
    let result = c_degrees * 9 / 5 + 32;
    result
}
