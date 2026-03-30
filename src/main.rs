/*
This is just me learning how to use rust
Don't expect anything good out of this
*/

use std::io;

fn main() {
    let options = [
        "Control Flow",
        "Ownership",
        "References and Borrowing",
        "Slices",
        "Structs",
        "Rectangles"
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
            1 => control_flow(),
            2 => ownership(),
            3 => references_and_borrowing(),
            4 => slices(),
            5 => structs(),
            6 => rectangles(),
            _ => println!("Please input a number between 1 and {options_size} inclusive.")
        }
    };

    println!("{result}");
}

// Structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // Normal struct use
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

// Tuple structs
/*
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
 */

// Unit-Like structs
// struct AlwaysEqual;

// Functions
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
fn references_and_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    println!("s2 = {s2}");
    change(&mut s2);
    println!("s2 = {s2}");
    /*
    Cannot have multiple mutable references to a variable.
    Similar to the atomic stuff with databases, prevents modifying something at the same time during async operations.
     */
    let mut s3 = String::from("hello");
    println!("s3 = {s3}");
    {
        let r1 = &mut s3;
        change(r1);
        println!("r1 = {r1}");
    }
    let r2 = &mut s3;
    change(r2);
    println!("r2 = {r2}");
    /*
    Can just use curly braces to add a section for having multiple mutable references.
    Same with C++, defines its own little scope.
    Don't think I will end up using this loads, but still important to know.
     */
    let final_string = no_dangle();
    println!("final_string = {final_string}");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn no_dangle() -> String {
    let string = String::from("hello");
    string

    // Return the value of string like this rather than as a reference prevents a dangling pointer.
}
fn slices() {
    // Don't do it like this.
    let mut s1 = String::from("hello world");
    let word1 = first_words(&s1);
    s1.clear();
    println!("The first word is {word1}");

    // Do it like this.
    let s2 = String::from("hello world");
    let word2 = better_first_words(&s2);
    // s2.clear() won't work here, since it would be a mutable borrow inbetween two immutable borrows on the lines above and below.
    println!("The first word is {word2}");
    /*
    The .. syntax is for a range, and the number before and after the .. relates to the start and end index of the slice.
    If you are including index 0 of the string, you could go &s[..5]. Same with the last character, &s[0..].
    A whole slice would just be &s[..]
     */
}
fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
} // Example of a bad way to do this.
fn better_first_words(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn structs() {
    let user1 = User {
        active: true,
        username: String::from("BillyBob"),
        email: String::from("BillyBob@gmail.com"),
        sign_in_count: 1,
    };

    let user_email = user1.email;
    println!("Email is {user_email}.");

    let mut user2 = User {
        active: true,
        username: String::from("JohnJones"),
        email: String::from("JohnJones@gmail.com"),
        sign_in_count: 4,
    };
    let mut user_name = user2.username;
    println!("Username is {user_name}.");
    user2.username = String::from("JohnnyJones");
    user_name = user2.username;
    println!("Username is now {user_name}.");

    let user3 = build_user(String::from("Bob"),String::from("Bob@bob.com"));

    let user4 = User {
        active: user1.active,
        username: user3.username,
        email: String::from("Email@email.com"),
        sign_in_count: user2.sign_in_count,
    };

    let user5 = User {
        email: String::from("Cheese@cheddar.com"),
        ..user4 // Taking all of the other fields from user4 using the range syntax.
    };

    // This is just to stop any warnings coming up for not using variables.
    let name = user5.username;
    let email = user5.email;
    let sign_in_count = user2.sign_in_count.to_string();
    let active = user3.active.to_string();
    println!("User {name} has the email {email}. They have signed in {sign_in_count} time/s.\
    If you were to say they are active, you would be {active}.");

    /*
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
     */
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width");
    }
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}