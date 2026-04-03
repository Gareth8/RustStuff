/*
This is just me learning how to use rust
Don't expect anything good out of this
*/
mod utils;
use std::io;
// use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    let options = [
        "Control Flow",
        "Ownership",
        "References and Borrowing",
        "Slices",
        "Structs",
        "Rectangles",
        "Enums",
        "Control Flow 2",
        "Collections",
        "Error Handling",
        "Generics",
    ];
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
            1 => utils::control_flow(),
            2 => utils::ownership(),
            3 => utils::references_and_borrowing(),
            4 => utils::slices(),
            5 => utils::structs(),
            6 => utils::rectangles(),
            7 => utils::enums(),
            8 => utils::control_flow_two(),
            9 => utils::collections(),
            10 => utils::error_handling(),
            11 => utils::generics(),
            _ => println!("Please input a number between 1 and {options_size} inclusive.")
        }
    };

    println!("{result}");
}

/* Structs
/*
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
*/

// Tuple structs
/*
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
 */

// Unit-Like structs
// struct AlwaysEqual;

// Enums
/*
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {

    }
}
*/
*/