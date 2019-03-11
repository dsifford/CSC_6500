mod data;
mod trial;

fn main() {
    let (cost, combination) = trial::brute_force();

    println!("Brute force:");
    println!("  Lowest cost = {}", cost);
    println!("  Best combination = {:?}", combination);

    for i in ([10, 100, 1_000, 10_000] as [usize; 4]).iter() {
        let (cost, combination) = trial::monte_carlo(*i);
        println!("Monte carlo ({} iterations):", i);
        println!("  Lowest cost = {}", cost);
        println!("  Best combination = {:?}", combination);
    }
}
