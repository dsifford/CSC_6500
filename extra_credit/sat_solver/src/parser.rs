use std::io::{BufRead, Result};

use crate::dpll::{Clause, Literal, Problem};

pub fn parse(reader: Box<BufRead>) -> Result<Problem> {
    reader
        .lines()
        .filter_map(|l| {
            if let Ok(line) = l {
                if ! &line.is_empty() {
                    return Some(Ok(parse_line(line)))
                }
            }
            None
        })
        .collect::<Result<Problem>>()
}

fn parse_line(line: String) -> Clause {
    line.split_whitespace()
        .map(
            |lit| match lit.chars().collect::<Vec<char>>().split_first().unwrap() {
                ('-', rest) | ('!', rest) => Literal {
                    atom: rest.iter().collect(),
                    value: false,
                },
                _ => Literal {
                    atom: lit.to_string(),
                    value: true,
                },
            },
        )
        .collect()
}
