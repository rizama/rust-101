#[test]
fn static_typing() {
    let name: &str = "sam";
    println!("Name: {}", name);

    // name = "Rizky";
    println!("Name: {}", name);
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

#[test]
fn numeric_operations() {
    let a: i32 = 10;
    let b: i32 = 3;
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
    println!("Remainder: {}", a % b);
    println!("Negation: {}", b - a);

    // augmented assignment
    // must be mut, because the value of x will be changed
    let mut x = 10;
    x += 1;
    println!("Augmented assignment: {}", x);

    x *= 2;
    println!("Augmented assignment: {}", x);

    x /= 2;
    println!("Augmented assignment: {}", x);

    x %= 3;
    println!("Augmented assignment: {}", x);
}

#[test]
fn test_boolean() {
    let a = true; // implicit type inference
    let b: bool = false; // explicit type declaration
    println!("Boolean: {}", a);
    println!("Boolean: {}", b);
}

#[test]
fn comparition_operator() {
    let a = 10;
    let b = 20;
    println!("Comparison: {}", a == b);

    println!("Comparison: {}", a != b);

    println!("Comparison: {}", a > b);
    println!("Comparison: {}", a < b);
    println!("Comparison: {}", a >= b);
    println!("Comparison: {}", a <= b);
}

#[test]
fn boolean_operator() {
    let a = true;
    let b = false;
    println!("Boolean operator: {}", a && b);
    println!("Boolean operator: {}", a || b);
    println!("Boolean operator: {}", !a);
}

#[test]
fn test_character() {
    let a: char = 'a';
    println!("Character: {}", a);
}