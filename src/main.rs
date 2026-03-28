// This is just me learning how to use rust
// Don't expect anything good out of this

fn main() {
    println!("Hello, world!");
    let result = add_two_nums(2, 3).to_string();
    println!("This is the result {result}")
}

fn add_two_nums (x: i32, y: i32) -> i32 {
    x + y
}