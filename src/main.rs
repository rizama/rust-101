fn main() {
    println!("Hello, world!");

    println!("Result: {}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn hello_test() {
    println!("Hello, world test!");
}

#[test]
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
fn static_typing() {
    let name: &str = "sam";
    println!("Name: {}", name);

    // name = "Rizky";
    println!("Name: {}", name);
}

#[test]
fn demonstrate_shadowing() {
    let y = 10; // First declaration of y
    println!("The initial value of y is: {}", y);

    let y = y + 5; // Shadowing y with a new value
    println!("After adding 5, the value of y is: {}", y);

    let x = 0;

    {
        let y = y * 3; // Shadowing y again within a new scope
        println!("In the inner scope, the value of y is: {}", y);

        let x = x + 1;
        println!("In the inner scope, the value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    println!("Outside the inner scope, the value of y is: {}", y); // y returns to the value from the outer scope
}

#[test]
fn test_number() {
    let a: i32 = 1;
    println!("Number i32: {}", a);

    let b: u32 = 1;
    println!("Number u32: {}", b);

    let c: f32 = 1.1;
    println!("Number f32: {}", c);

    let d: isize = 1;
    println!("Number isize: {}", d);

    let e: usize = 1;
    println!("Number usize: {}", e);

    // convertion number
    let f: i32 = 1;
    let g: u32 = f as u32;
    println!("Number i32 to u32: {}", g);

    let h: i64 = g as i64;
    println!("Number i32 to i64: {}", h);

    let i: i64 = 1000000000;
    let j: i8 = i as i8; // integer overflow, because i8 only can store -128 to 127
    println!("Number i64 to i8: {}", j);
}
