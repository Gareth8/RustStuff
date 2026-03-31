enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// Functions
pub fn enums() {
    /*
    let m = Message::Write(String::from("hello"));
    m.call();
     */
    /*
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    */
    let penny = value_in_cents(Coin::Penny).to_string();
    let quarter = value_in_cents(Coin::Quarter).to_string();
    let dime = value_in_cents(Coin::Dime).to_string();
    let nickel = value_in_cents(Coin::Nickel).to_string();
    println!("The penny has a value of {penny}.\
    The quarter has a value of {quarter}.\
    The dime has a value of {dime}.\
    The nickel has a value of {nickel}.");
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}