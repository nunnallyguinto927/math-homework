use std::collections::HashMap;

fn main() {
    let mut nums = HashMap::new();
    nums.insert(1, 2);
    nums.insert(3, 4);

    for (key, value) in nums.iter_mut() {
        *value *= 5;
    }

    println!("{:?}", nums);
}
