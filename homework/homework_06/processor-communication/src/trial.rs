extern crate rand;

use crate::data;
use rand::distributions::{Distribution, Uniform};

pub fn brute_force() -> (u32, Vec<usize>) {
    let mut cost: u32 = 99999;
    let mut combination: Vec<usize> = vec![];
    for p1 in 0..4 {
        for p2 in 0..4 {
            for p3 in 0..4 {
                for p4 in 0..4 {
                    for p5 in 0..4 {
                        let this_cost = data::processor_cost(p1, p2, p3, p4, p5)
                            + data::communication_cost(p1, p2, p3, p4, p5);
                        if this_cost < cost {
                            cost = this_cost;
                            combination = vec![p1, p2, p3, p4, p5];
                        }
                    }
                }
            }
        }
    }
    (cost, combination)
}

pub fn monte_carlo(iterations: usize) -> (u32, Vec<usize>) {
    let mut cost: u32 = 99999;
    let mut combination: Vec<usize> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let (this_cost, this_combination) = mc_trial(&mut rng);
        if this_cost < cost {
            cost = this_cost;
            combination = this_combination;
        }
    }

    (cost, combination)
}

fn mc_trial(rng: &mut rand::prelude::ThreadRng) -> (u32, Vec<usize>) {
    let options = Uniform::from(0..4);
    let sample: Vec<usize> = options.sample_iter(rng).take(5).collect();
    let cost = data::processor_cost(sample[0], sample[1], sample[2], sample[3], sample[4])
        + data::communication_cost(sample[0], sample[1], sample[2], sample[3], sample[4]);
    (cost, sample)
}
