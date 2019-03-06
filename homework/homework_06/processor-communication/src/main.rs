
mod data;

fn brute_force() -> (u32, Vec<usize>) {
    let mut cost: u32 = 99999;
    let mut combination: Vec<usize> = vec![];
    for p1 in 0..4 {
        for p2 in 0..4 {
            for p3 in 0..4 {
                for p4 in 0..4 {
                    for p5 in 0..5 {
                        let this_cost = data::processor_cost(p1, p2, p3, p4, p5) + data::communication_cost(p1, p2, p3, p4, p5);
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

fn main() {
    let (cost, combination) = brute_force();

    println!("Lowest cost = {}", cost);
    println!("Best combination = {:?}", combination);
}
