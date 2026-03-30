/*
This is just me learning how to use rust
Don't expect anything good out of this
*/

use std::io;

fn main() {
    let options = ["Control Flow", "Ownership"];
    let mut count = 1;
    println!("Choose one of the following options:");

    for option in options {
        println!("{count}: {option}");
        count += 1;
    }

    println!("Type the word 'quit' to exit");

    let result = loop {
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        if user_choice.trim() == "quit" { // Forgot to trim user_choice to remove the newline characters. Added that.
            break "Finished!";
        }

        let user_choice: u32 = user_choice
            .trim()
            .parse()
            .unwrap_or_else(|_| 0);

        let options_size = options.len();

        match user_choice {
            1 => control_flow(),
            2 => ownership(),
            _ => println!("Please input a number between 1 and {options_size}.")
        }
    };

    println!("{result}");
}

fn add_two_nums (x: i32, y: i32) -> i32 {
    x + y
}
fn control_flow() {
    let new_number = add_two_nums(4,2).to_string();
    println!("The result of 4 + 2 is {}", new_number);

    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break "Finished";
        }
        println!("Counter: {}", counter);
        counter += 1;
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let new_string = takes_and_gives_back(String::from("Cheese"));
    println!("{new_string}");
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}