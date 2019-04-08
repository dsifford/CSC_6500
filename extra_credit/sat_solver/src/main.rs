use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use clap::clap_app;

mod dpll;
mod parser;

fn main() {
    let args = clap_app!(sat_solver =>
       (version: "0.1.0")
       (author: "Derek Sifford <dereksifford@gmail.com>")
       (about: "3SAT problem solver using a slightly modified DPLL algorithm")
       (@arg FILE: +required {validate_file_arg}
        "Solves 3SAT defined in FILE.\n\
         If FILE is '-', read from stdin."
       )
    )
    .get_matches();

    let reader: Box<BufRead> = match args.value_of("FILE").unwrap() {
        "-" => Box::new(BufReader::new(io::stdin())),
        file => Box::new(BufReader::new(File::open(file).unwrap())),
    };

    let problem = parser::parse(reader).unwrap();
    dpll::solve(problem);
}

fn validate_file_arg(arg: String) -> Result<(), String> {
    match arg.as_ref() {
        "-" => Ok(()),
        file => {
            if Path::new(file).is_file() {
                Ok(())
            } else {
                Err(format!("File '{}' does not exist.", file))
            }
        }
    }
}
