use std::rand;
fn main() {
    let num = rand::thread_rng().gen_range(1..=10);
    println!("Random number between 1 and 10: {}", num);
}
