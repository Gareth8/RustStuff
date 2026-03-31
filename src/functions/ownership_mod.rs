pub fn ownership() {
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