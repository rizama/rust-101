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

#[test]
fn string_type() {
    // ref: https://doc.rust-lang.org/std/primitive.str.html
    // string slice = &str => fixed size => save in stack
    // string slice is an immutable reference to a string and cannot be changed
    // if we change/update the value of a, original string will not be changed
    // but it will be replaced by the new value of a
    let mut a: &str = "RIZKY"; // value still exist in stack
    println!("String slice: {}", a);
    a = "SAM"; // set new value but same variable
    println!("String slice: {}", a);

    let name = "     Rizky     ";
    let trimmed_name = name.trim();
    println!("name: {}", name);
    println!("Trimmed name: {}", trimmed_name);

    println!("--------------------------------");

    // ref: https://doc.rust-lang.org/std/string/struct.String.html
    // string = String => dynamic size => save in heap
    let mut a: String = String::from("RIZKY");
    a.push_str(" SAM");
    println!("String: {}", a);

    let b = a.replace("SAM", "PRATAMA");
    println!("String replace: {}", b);
}

#[test]
fn ownership_rules() {
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // 'a' variable cannot access here, because have not been declared
    let a = 10; // variable 'a' is the owner of the value 10 and can access start from here

    {
        // variable 'b' cannot access here, because have not been declared
        let b = 20; // variable 'b' is the owner of the value 20 and can access start from here
        println!("Variable b: {}", b);
    } // variable 'b' goes out of scope and the value will be dropped and cannot access anymore

    println!("Variable a: {}", a);
} // variable 'a' goes out of scope and the value will be dropped and cannot access anymore

#[test]
fn data_copy() {
    // Copy data only for All data type fixed size (saved in stack)

    // for All data type fixed size (saved in stack) if we assign a value to another variable,
    // the value will be copied to the new variable, not reference
    let a = 10;

    // 'b' is a copy of 'a'
    // 'b' is a new variable with its own value
    // if we change the value of 'b', the value of 'a' will not be changed
    let mut b = a;
    println!("Variable b: {}", b);

    b = 100;

    println!("Variable a: {}", a);
    println!("Variable b: {}", b);
}

#[test]
fn ownership_move() {
    // Move data only for All data type dynamic size (saved in heap)

    let a = String::from("RIZKY");
    println!("Variable a: {}", a);

    let b = a; // 'b' is the owner of the value and 'a' is no longer the owner, and 'a
    println!("Variable b: {}", b);

    // println!("Variable a: {}", a); // this will cause an error
}

#[test]
fn clone_data() {
    // We can use clone() to copy the value of a variable to another variable for All data type dynamic size (saved in heap)
    // Clone data only for All data type dynamic size (saved in heap)
    // Clone data is a deep copy of the value
    // if we change the value of 'b', the value of 'a' will not be changed
    // process of cloning is expensive, because it will create a new copy of the value

    let a = String::from("RIZKY");
    let b = a.clone();
    println!("Variable a: {}", a);
    println!("Variable b: {}", b);
}

#[test]
fn if_expression() {
    let a = 6;
    if a >= 8 {
        println!("Good");
    } else if a >= 6 {
        println!("Not bad");
    } else {
        println!("Try again");
    }

    // Let statement
    let c = 7;
    let result: &str = if c >= 8 {
        "Good"
    } else if c >= 6 {
        "Not bad"
    } else {
        "Try again"
    };
    println!("Result: {}", result);
}

#[test]
fn loop_expression() {
    let mut a = 0;
    loop {
        a += 1;
        if a == 5 {
            continue;
        }
        println!("Loop: {}", a);

        if a >= 10 {
            break;
        }
    }

    // return value from loop
    let mut a = 0;
    let result = loop {
        a += 1;
        if a == 10 {
            break a * 2; // return value from loop
        }
    };
    println!("Result: {}", result);

    // loop label
    let mut number: i32 = 1;
    'outer: loop {
        // label 'outer'
        let mut x = 1;
        loop {
            if number == 11 {
                break 'outer;
            }

            println!("{} x {} = {}", number, x, number * x);

            x += 1;
            if x == 11 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_expression() {
    let mut a = 0;
    while a < 10 {
        println!("While: {}", a);
        a += 1;
    }
}

#[test]
fn iterate_array() {
    // manual
    let z: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < z.len() {
        println!("Element: {}", z[index]);
        index += 1;
    }

    // for in
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("Element: {}", element);
    }

    // iterate array of object
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("Element: {}", element);
    }
}

#[test]
fn range_expression() {
    // for range
    let range = 0..5;
    println!("Start Range: {:?}", range.start); // index include
    println!("End Range: {:?}", range.end); // index exclude
    for i in 0..5 {
        println!("Range exclude end: {}", i);
    }

    // range include
    let range_include = 0..=5;
    println!("Start Range include: {:?}", range_include.start()); // index include
    println!("End Range include: {:?}", range_include.end()); // index include
    for i in range_include {
        println!("Range include end: {}", i);
    }

    // for range with step
    for i in (0..5).step_by(2) {
        println!("Range with step: {}", i);
    }
}
