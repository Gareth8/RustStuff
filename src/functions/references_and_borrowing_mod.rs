pub fn references_and_borrowing() {
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