use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let number: i32 = match args[1].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("Error: argument must be a single integer");
                    return;
                }
            };
            let (final_num, iterations) = times3(number);
            println!("Final number is {} after {} iterations", final_num, iterations);
        },
        _ => {
            eprintln!("Error: argument must be a single integer");
            return;
        }
    }
}

fn times3(num: i32) -> (i32, i32) {
    let mut output: i32 = num;
    let mut iterations: i32 = 0;
    while output != 1 {
        iterations += 1;
        match output {
            n if n % 2 == 0 => {
                output /= 2;
            },
            _ => {
                output = output * 3 + 1;
            }
        }
        if (output & (output - 1)) == 0 {
            break;
        }
    }
    return (output, iterations);
}
