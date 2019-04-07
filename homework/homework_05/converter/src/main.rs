use clap::clap_app;

use machines::{b2u, u2b};

mod machines;
mod tm;

fn main() {
    let args = clap_app!(Converter =>
        (version: "0.1.0")
        (author: "Derek Sifford <dereksifford@gmail.com>")
        (about: "Turing machines to convert binary to unary and vice-versa")
        (@arg b2u: -b --b2u +required +takes_value conflicts_with[u2b] "Convert from binary input")
        (@arg u2b: -u --u2b +required +takes_value conflicts_with[b2u] "Convert from unary input")
    )
    .get_matches();

    if args.is_present("b2u") {
        let value: i32 = args.value_of("b2u").unwrap().parse().unwrap();
        let input = format!("{:b}", value);

        println!("Running for input {} (binary {})", value, input);

        let converted = b2u::run(&input).unwrap();
        println!("\tb2u = {}", converted);

        let converted = u2b::run(&converted).unwrap();
        println!("\tu2b = {}", converted);
    } else {
        let value: usize = args.value_of("u2b").unwrap().to_string().parse().unwrap();
        let input = "1".repeat(value);

        println!("Running for input {} (unary {})", value, input);

        let converted = u2b::run(&input).unwrap();
        println!("\tu2b = {}", converted);

        let converted = b2u::run(&converted).unwrap();
        println!("\tb2u = {}", converted);
    };
}
