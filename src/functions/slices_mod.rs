pub fn slices() {
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