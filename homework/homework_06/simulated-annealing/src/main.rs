extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use rand::thread_rng;

fn main() {
    let result = minimize();
    println!("Optimum value of x = {}", result)
}

fn minimize() -> i32 {
    let mut rng = thread_rng();
    let mut result: i32 = random_num(&mut rng);

    for T in (10..101).rev().step_by(10) {
        for _ in 0..4 {
            let x1 = result;
            let x2 = random_num(&mut rng);
            if ((-(E(x2) - E(x1)) / T) as f32).exp() > 1.0 {
                result = x2;
                break;
            }
        }
    }
    result
}

fn E(x: i32) -> i32 {
    x.pow(2) - (15 * x) + 54
}

fn random_num(rng: &mut ThreadRng) -> i32 {
    let options = Uniform::from(0..99);
    options.sample(rng)
}
