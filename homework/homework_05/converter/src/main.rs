mod machines;
mod tm;

use machines::{b2u, u2b};

fn main() {
    let input = 45;
    let binary_str = format!("{:b}", input);
    println!("Running for input {} (binary {})", input, binary_str);

    let converted = b2u::run(&binary_str).unwrap();
    println!("b2u = {}", converted);

    let converted = u2b::run(&converted).unwrap();
    println!("u2b = {}", converted);
}
