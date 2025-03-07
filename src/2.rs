  pub fn random_code() {
let mut rng = rand::thread_rng();
let secret_number = rng.gen_range(1..=10);
if secret_number == 5 {
return "Correct!".to_string();
} else {
return format!("Incorrect, the number is {}", secret_number);
}}