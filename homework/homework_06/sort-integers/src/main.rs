use std::env;
use std::time::Instant;

mod sort;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let size: u32 = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Argument must be an integer.");
                    return;
                }
            };
            let list = utils::shuffled_list(size);

            println!("Standard Quicksort (size {}):", size);
            let now = Instant::now();
            sort::quicksort(list.clone(), false);
            println!("  Duration: {} nanoseconds", now.elapsed().as_nanos());

            println!("Randomized Quicksort (size {}):", size);
            let now = Instant::now();
            sort::quicksort(list.clone(), true);
            println!("  Duration: {} nanoseconds", now.elapsed().as_nanos());
        }
        _ => eprintln!("Must pass a single integer argument."),
    }
}
