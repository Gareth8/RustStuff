use RustStuff::add_two;

#[test]
fn test_add_two() {
    let result = add_two(2);
    assert_eq!(result, 4)
}