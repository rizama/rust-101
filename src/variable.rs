fn test_variable() {
    let a: i32 = -1;
    let b: i32 = 2;
    let mut name = "Rizky";
    println!("Name: {}", name);
    name = "Sam";
    assert_ne!(a, b);
    println!("Name: {}", name);
}

#[test]
fn run_test_variable() {
    test_variable();
}