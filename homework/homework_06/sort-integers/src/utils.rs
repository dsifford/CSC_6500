extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn random_idx(length: usize) -> usize {
    thread_rng().gen_range(0, length)
}

pub fn shuffled_list(size: u32) -> Vec<u32> {
    let mut list: Vec<u32> = (0..size).collect();
    list.shuffle(&mut thread_rng());
    list
}
