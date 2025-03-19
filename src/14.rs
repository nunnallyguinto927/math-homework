fn main() {
    let number = 5;
    let result = my_function(number);
    println!("Result: {}", result);
}

fn my_function(n: i32) -> i32 {
    n * 2 + 1
}
