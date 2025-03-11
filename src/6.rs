fn main() {
    let mut rng = rand::thread_rng();
    let num1: i32 = rng.gen_range(0..100);
    let num2: i32 = rng.gen_range(0..100);
    let result = num1 + num2;
    println!("{}", result);
}