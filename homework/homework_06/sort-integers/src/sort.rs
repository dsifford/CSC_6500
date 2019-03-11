extern crate rand;

use crate::utils::random_idx;

pub fn quicksort(list: Vec<u32>, random_pivot: bool) -> Vec<u32> {
    match list.len() {
        0...1 => list,
        length => {
            let idx = if random_pivot { random_idx(length) } else { 1 };
            let pivot = list[idx];
            let (left, right): (Vec<u32>, Vec<u32>) = [&list[..idx], &list[(idx + 1)..]]
                .concat()
                .into_iter()
                .partition(|&i| i < pivot);
            [
                quicksort(left, random_pivot),
                vec![pivot],
                quicksort(right, random_pivot),
            ]
            .concat()
        }
    }
}
