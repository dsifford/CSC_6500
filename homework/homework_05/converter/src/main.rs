extern crate clap;

mod machines;
mod tm;

use clap::{App, Arg};
use machines::{b2u, u2b};

fn main() {
    let args = App::new("Converter")
        .version("0.1.0")
        .author("Derek Sifford <dereksifford@gmail.com>")
        .about("Turing machines to convert binary to unary and vice-versa")
        .arg(
            Arg::with_name("b2u")
                .required(true)
                .long("b2u")
                .short("b")
                .takes_value(true)
                .help("Convert from binary input")
                .conflicts_with("u2b"),
        )
        .arg(
            Arg::with_name("u2b")
                .required(true)
                .long("u2b")
                .short("u")
                .takes_value(true)
                .help("Convert from unary input")
                .conflicts_with("b2u"),
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
