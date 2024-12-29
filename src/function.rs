fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn hello_test() {
    println!("Hello, world test!");
}

#[test]
fn run_function() {
    let result = add(1, 2);
    println!("1 + 2 = {}", result);

    hello_test();
}
