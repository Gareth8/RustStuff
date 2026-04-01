use std::fs::File;
use std::io::ErrorKind;
// use std::io;
// use std::fs;
pub fn error_handling() {
    // panic!("Crash and Burn");

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        _ => {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }