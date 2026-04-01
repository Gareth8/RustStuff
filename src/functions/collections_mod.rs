use std::collections::HashMap;
/*
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
*/
pub fn collections() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let mut v = vec![1,2,3];

    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    /*
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    */
}
fn strings() {
    let mut s1 = String::from("initial content");
    s1.push_str(" + content");
    s1.push('D');
    let s2 = String::from("poop");
    let s3 = format!("{s1}-{s2}");
    println!("s3 = {s3}");
    let s4 = &s3[..5];
    println!("s4 = {s4}");
    for c in s4.chars() {
        println!("{c}");
    }
}
fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}