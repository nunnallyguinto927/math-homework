use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(0..10);
    println!("The random number is: {}", x);
}
