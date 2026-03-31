struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // Normal struct use
pub fn structs() {
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