// ref: https://docs.google.com/presentation/d/1Z3u_470twXmqxo9YmpTYHYYae8lKwVnb0czvXvdTn4I/edit

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

// SCALAR TYPES

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

// COMPOUND TYPES
fn test_unit() {
    println!("Hello Unit");
}

#[test]
fn tuple_type() {
    // tuple is a collection of values of different types
    // the number of values in a tuple is fixed
    // use {:#?} to print the tuple
    // by default, tuple is immutable
    let a: (i32, f64, char, bool) = (1, 2.0, 'a', true);
    println!("Tuple: {:#?}", a);

    // access the value of a tuple
    let b = a.0;
    println!("Tuple index 0: {}", b);

    let c = a.1;
    println!("Tuple index 1: {}", c);

    // destructuring a tuple
    // _ is used to ignore the value of the tuple
    let (d, e, f, _) = a;
    println!("Tuple destructuring: {}", d);
    println!("Tuple destructuring: {}", e);
    println!("Tuple destructuring: {}", f);
    // println!("Tuple destructuring: {}", g);

    // mutable tuple
    let mut a = (1, 2, 3);
    a.0 = 4;
    println!("Tuple mutable: {:#?}", a);

    // tuple unit
    // tuple unit is a tuple with no values
    // this used to indicate that the function returns nothing
    let a: () = ();
    println!("Tuple unit: {:#?}", a);

    let result = test_unit();
    println!("Result: {:?}", result);
}

#[test]
fn array_type() {
    // array is a collection of values of the same type
    // the number of values in an array is fixed
    // use {:#?} to print the array
    // by default, array is immutable
    let a: [i32; 3] = [1, 2, 3];
    println!("Array immutable: {:#?}", a);

    // access the value of an array
    let b = a[0];
    println!("Array index 0: {}", b);

    // mutate the value of an array
    let mut a: [i32; 3] = [1, 2, 3];
    a[0] = 4;
    println!("Array mutated: {:#?}", a);

    // length of an array
    let a: [i32; 3] = [1, 2, 3];
    let length: usize = a.len();
    println!("Length of array: {}", length);

    // two dimensional array
    let a: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("Two dimensional array: {:#?}", a);

    // access the value of a two dimensional array
    let b = a[0][0];
    println!("Two dimensional array index 0 0: {}", b);
}

const MAXIMUM_NUMBER: i32 = 1000;
#[test]
fn constant_type() {
    // constant is a value that is fixed and cannot be changed
    // by default, constant is immutable
    // constant must be declared with a type
    // constant name must be in uppercase
    // constant can be declared outside of a function
    const PI: f64 = 3.14;
    println!("Constant: {}", PI);

    // constant cannot be assigned by other calculation
    // let a = 1 + 2;
    // const A: i32 = a;
    // println!("Constant: {}", A);

    println!("Maximum number: {}", MAXIMUM_NUMBER);
}

// SCOPE AND BLOCKS

#[test]
fn variable_scope() {
    println!("Maximum number: {}", MAXIMUM_NUMBER);

    let name = "Sam";
    {
        println!("Outer Name print in inner scope: {}", name);
        let last_name = "Pratama";
        println!("Variable inner scope: {}", last_name);
    }
    println!("Name: {}", name);
    // println!("Variable scope: {}", last_name); // this will cause an error
}

// Memory Allocation
// Stack and Heap

#[test]
fn stack_and_heap() {
    function_stack_and_heap_a();
    function_stack_and_heap_b();
}

fn function_stack_and_heap_a() {
    let a = 10; // save in stack
    let b = String::from("RIZKY"); // save in heap

    println!("{} {}", a, b);
}

fn function_stack_and_heap_b() {
    let a = 10; // save in stack
    let b = String::from("SAM"); // save in heap

    println!("{} {}", a, b);
}
