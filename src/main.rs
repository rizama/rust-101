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

// FUNCTION

fn say_hello_() {
    println!("Hello");
}

fn say_hello_to(name: &str, age: i32) {
    println!("Hello {} and age {}", name, age);
}

fn sum(a: i32, b: i32) -> i32 {
    // return value
    a + b
}

fn factorial_loop(n: i64) -> i64 {
    if n < 0 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    // return result;
    // or just call result
    // code in the last line will be returned
    result
}

fn print_text(value: String, times: i32) {
    if times <= 0 {
        return;
    }

    println!("{}", value);

    print_text(value, times - 1);
}

fn recursive_factorial_loop(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    n * recursive_factorial_loop(n - 1)
}

#[test]
fn feature_function() {
    say_hello_();
    say_hello_to("Rizky", 29);
    println!("Sum: {}", sum(1, 2));
    println!("Factorial loop: {}", factorial_loop(5));
    print_text(String::from("SAM"), 5);
    println!("Recursive factorial loop: {}", recursive_factorial_loop(5));
}

fn print_number(n: i32) {
    println!("Number + 1: {}", n + 1);
}

fn hi(name: String) {
    println!("Hi {}", name);
}

fn full_name(first_name: String, last_name: String) -> String {
    // if return type is String (or other type saved in heap), the value will be moved to the caller
    // if return type is &str (or other type saved in stack), the value will be copied to the caller

    // in this case, the value of this function will be moved to the caller, called'name'
    format!("{} {}", first_name, last_name)
}

fn full_name_bring_back_ownership(
    first_name: String,
    last_name: String,
) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    // bring back ownership to the caller
    (first_name, last_name, full_name)
}

#[test]
fn function_ownership() {
    // for data type fixed size (saved in stack), the value will be copied to the parameter
    let a = 10;
    print_number(a);
    println!("Number a: {}", a);

    // for data type dynamic size (saved in heap), the value will be moved to the parameter
    // and the original value will be invalidated
    let name = String::from("Rizky");
    hi(name);
    // println!("Name: {}", name); // this will cause an error

    // return value ownership
    let first_name = String::from("Rizky");
    let last_name = String::from("Sam");
    let name = full_name(first_name, last_name); // value first_name and last_name will be moved to the parameter of the full_name function
    println!("Full name: {}", name);

    // println!("First name: {}", first_name); // this will cause an error, because the value of first_name has been moved to the parameter of the full_name function
    // println!("Last name: {}", last_name); // this will cause an error, because the value of last_name has been moved to the parameter of the full_name function

    // returning ownership
    let f_name = String::from("Sam");
    let l_name = String::from("Pratama");

    // the function will return ownership to the caller
    let (f_name, l_name, full_name) = full_name_bring_back_ownership(f_name, l_name);
    println!("Full name: {}", full_name);
    println!("First name: {}", f_name);
    println!("Last name: {}", l_name);
}

fn full_name_with_reference(first_name: &mut String, last_name: &String) -> String {
    // we can update the value of first_name, because it's a mutable reference
    // *first_name  = String::from("Ariel");
    first_name.push_str(" Ariel");

    format!("{} {}", first_name, last_name)
}

#[test]
fn function_reference() {
    let mut a = 10;
    let b = &mut a;
    *b = 1;
    println!("Variable b: {}", b);
    println!("Variable a: {}", a);

    let mut first_name = String::from("Rizky");
    let last_name = String::from("Sam");

    // with mutable reference
    // if we want to change the value of first_name, we need to use mutable reference
    let full_name = full_name_with_reference(&mut first_name, &last_name);

    println!("Full name: {}", full_name);
    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);

    // mutable reference only can be used once
    // if we want to use mutable reference again, we need to declare it again
    let mut value = String::from("Rizky");
    let borrow_value = &mut value;
    // let borrow_value2 = &mut value; // this will cause an error
    println!("Borrow value: {}", borrow_value);

    // Rules of references
    // You can have either:
    //     One mutable reference (&mut T)
    //     OR any number of immutable references (&T)
    //     But never both at the same time
    let mut value2 = String::from("Sam");
    let borrow_value3 = &mut value2;
    // let borrow_value4 = &value2; // this will cause an error
    println!("Borrow value2: {}", borrow_value3);
}

// dangling pointer
// ref: https://doc.rust-lang.org/book/ch04-03-dangling-pointers.html
#[test]
fn dangling_pointer() {
    println!("Dangling pointer: {}", dangle());
}

// this function will return a reference to a String that is local to the function
// but this will cause a dangling pointer error, because the String will be dropped when the function ends
// and the reference will point to an invalid memory location
// so we cant return a reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// solution for dangling pointer
fn dangle() -> String {
    let s = String::from("hello");
    s // Return the String itself, transferring ownership
}

#[test]
fn range_reference() {
    let a: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice: &[i32] = &a[1..3];
    println!("Slice: {:?}", slice);

    let slice2 = &a[..3];
    println!("Slice2: {:?}", slice2);

    let slice3 = &a[..];
    println!("Slice3: {:?}", slice3);

    let slice4 = &a[1..];
    println!("Slice4: {:?}", slice4);

    let slice5 = &a[..=3];
    println!("Slice5: {:?}", slice5);
}

#[test]
fn string_slice() {
    let a = String::from("Rizky Sam Pratama");

    let first = &a[0..5];
    println!("First: {}", first);

    let last = &a[10..];
    println!("Last: {}", last);
}

#[derive(Debug)]
struct Person {
    name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("Person name: {}", person.name);
    println!("Person middle name: {}", person.middle_name);
    println!("Person last name: {}", person.last_name);
    println!("Person age: {}", person.age);
}

#[test]
fn test_struct() {
    let person: Person = Person {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };

    println!("Person: {:#?}", person);

    print_person(&person);

    let persons = vec![
        Person {
            name: String::from("John"),
            middle_name: String::from("William"),
            last_name: String::from("Doe"),
            age: 25,
        },
        Person {
            name: String::from("Jane"),
            middle_name: String::from("Marie"),
            last_name: String::from("Smith"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            middle_name: String::from("James"),
            last_name: String::from("Johnson"),
            age: 35,
        },
    ];

    println!("All persons: {:#?}", persons);
    println!("1 persons: {:#?}", persons[0]);

    // init shorthand
    let name = String::from("Rizky 1");
    let age = 29;

    let sam: Person = Person {
        name, // shorthand for name: name, ownership is transferred to the struct
        middle_name: String::from("Sam 1"),
        last_name: String::from("Pratama 1"),
        age, // ownership is NOT transferred to the struct, because it's fixed size
    };

    println!("Person: {:#?}", sam);
    println!("Person name: {}", age);

    // struct update syntax
    // this will copy the value of sam -> samsul
    // but be careful for dinamyc type (String, Vec, etc, who stored in heap)
    // because the value will be moved to 'samsul'
    let mut samsul = Person { ..sam };

    samsul.name = String::from("Rizky 3");
    println!("Person: {:#?}", samsul);
    println!("Person name: {}", sam.age);
    // println!("Person middle name: {}", sam.name); // this will cause an error, because the value of sam.name has been moved to samsul

    // solution for dinamyc type (String, Vec, etc, who stored in heap)
    // we can use clone() method to copy the value
    let new_sam = Person {
        name: String::from("Rizky 4"),
        middle_name: String::from("Sam 4"),
        last_name: String::from("Pratama 4"),
        age: 30,
    };
    let samsul2 = Person {
        name: new_sam.name.clone(),
        middle_name: new_sam.middle_name.clone(),
        last_name: new_sam.last_name.clone(),
        ..new_sam
    };

    print_person(&samsul2);
    print_person(&new_sam);

    // tuple struct
    #[derive(Debug)]
    struct GeoPoint(f64, f64);

    let point = GeoPoint(10.0, 20.0);
    println!("Point: {:?}", point);
    println!("Point: {}", point.0);
    println!("Point: {}", point.1);

    // struct without field names
    struct Nothing;
    let _nothing = Nothing;
    let _nothing2 = Nothing {};
}

// Method struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // created method always have &self as first parameter
    // &self is a reference to the instance of the struct
    // this method means that the method will be called on the instance of the struct
    // if self is not reference, the ownership of the instance will be moved to the method (for type who stored in heap)
    // in this case, rect instance will be moved to the method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_name_shape(&self, name: &String) {
        println!("Shape name: {}", name);
    }
}

#[test]
fn test_method_struct() {
    let rect = Rectangle {
        height: 10,
        width: 20,
    };

    println!("Rectangle area: {}", rect.area());

    println!("Rectangle: {}", rect.height); // this will cause an error, because the value of rect has been moved to the method

    let name = String::from("Rectangle");
    rect.print_name_shape(&name);
}

// Associated functions
// associated functions are functions that are associated with the struct
// but not with the instance of the struct
// associated functions are defined using the impl keyword
// associated functions are called using the struct name and the :: operator
// associated functions are often used for constructors

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn baru(lat: f64, long: f64) -> GeoPoint {
        GeoPoint(lat, long)
    }
}

#[test]
fn associated_function() {
    let geo_point = GeoPoint::baru(-6.2000, 106.00000);
    println!("lat {}", geo_point.0);
    println!("long {}", geo_point.1);
}

// Enum

#[derive(Debug)]
enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn enum_feature() {
    let level = Level::Regular;
    let _level1 = Level::Premium;
    let _level2 = Level::Platinum;
    println!("Level: {:?}", level);
}

// Enum Data
#[derive(Debug)]
enum Payment {
    CreditCard(String),
    DebitCard(String, String),
    EWallet(String),
}

#[test]
fn enum_data() {
    let cc = Payment::CreditCard(String::from("1234567890"));
    println!("Payment: {:?}", cc);
    // if let statement
    // if let statement is used to match a value against an enum
    // how to read if let statement: if let <pattern> = <expression> { <body> }
    // memeriksa apakah cc adalah varian CreditCard dari enum Payment
    // jika ya, maka value akan diassign ke 'x'
    if let Payment::CreditCard(x) = cc {
        println!("Credit card value: {}", x);
    }

    let debit = Payment::DebitCard(String::from("BCA"), String::from("1234567890"));
    println!("Payment: {:?}", debit);

    let ewallet = Payment::EWallet(String::from("Dana"));
    println!("Payment: {:?}", ewallet);
}

// Enum method

impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying with {:?} amount: {}", self, amount);
    }
}

#[test]
fn enum_method() {
    let cc = Payment::CreditCard(String::from("1234567890"));
    cc.pay(30000);
}

// Pattern Matching

// 1. Pattern Matching Enum

#[test]
fn pattern_matching_enum() {
    let level1 = Level::Regular;
    let mut test: &str = "";
    match level1 {
        Level::Regular => test = "Regular",
        Level::Premium => println!("Premium level"),
        Level::Platinum => println!("Platinum level"),
    }

    println!("Test: {}", test);
}

// 2. Destructuring Enum Data Pattern
impl Payment {
    fn paying(&self, number: u32) {
        match self {
            Payment::CreditCard(x) => println!("Paying with credit card {}, amount: {}", x, number),
            Payment::DebitCard(bank, _x) => {
                println!("Paying with debit card {} {}, amount: {}", bank, _x, number)
            }
            Payment::EWallet(_x) => println!("Paying with e-wallet, amount: {}", number), // _x is ignored
        }
    }
}

#[test]
fn test_destructuring_enum_data_pattern() {
    let cc = Payment::CreditCard(String::from("1234567890"));
    cc.paying(30000);

    let debit = Payment::DebitCard(String::from("BCA"), String::from("1234567890"));
    debit.paying(30000);

    let ewallet = Payment::EWallet(String::from("Dana"));
    ewallet.paying(30000);
}

// 3. Pattern Matching Value

#[test]
fn pattern_matching_value() {
    let number = 10;
    match number {
        1 => println!("Number is 1"),
        2 => println!("Number is 2"),
        _ => println!("Number is not 1 or 2"),
    }

    let name: &str = "Pratama";
    match name {
        "Rizky" => println!("Name is Rizky"),
        "Sam" => println!("Name is Sam"),
        _other => println!("Name is not Rizky or Sam"),
    }
}

// 4. Multiple Matching
#[test]
fn multiple_matching() {
    let number = 2;
    match number {
        1 | 2 => println!("Number is 1 or 2"),
        _other => println!("Number is not 1 or 2"),
    }
}

// 5. Range Matching
#[test]
fn range_matching() {
    let number = 5;

    // 1..=5 means number is between 1 and 5
    match number {
        1..=5 => println!("Number is between 1 and 5"),
        _other => println!("Number is not between 1 and 5"),
    }
}

// 6. Destructuring Struct Pattern
#[test]
fn destructuring_struct_pattern() {
    let point = GeoPoint::baru(1.0, 0.0);

    match point {
        GeoPoint(0.0, long) => println!("long is {}", long),
        GeoPoint(lat, 0.0) => println!("lat is {}", lat),
        GeoPoint(lat, long) => println!("lat: {}, long: {}", lat, long),
    }

    let person = Person {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };

    match person {
        // .. is used to ignore the value of the field
        // ignore the value of middle_name, last_name
        Person { name, age, .. } => {
            println!("Name: {}, Age: {}", name, age);
        }
    }
}

// 7. Ignoring Value
#[test]
fn ignoring_value() {
    let point = GeoPoint::baru(1.0, 1.0);

    match point {
        GeoPoint(lat, _) => println!("lat is {}", lat),
    }
}

// 8. Match Expression
#[test]
fn match_expression() {
    let number = 10;
    let result = match number {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    println!("Result: {}", result);
}

// Alias

type Age = u8;

#[test]
fn alias() {
    let age: Age = 29;
    println!("Age: {}", age);
}

// Module
mod module_a {
    // all code in this block will be private
    // unless we use 'pub' keyword

    pub struct User {
        pub name: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is, {}", name, self.name);
        }
    }
}

use module_a::User;

#[test]
fn test_module() {
    let person = User {
        name: String::from("Rizky"),
        age: 29,
    };
    println!("Person age: {}", person.age);
    person.say_hello("Sam");
}

// USE KEYWORD
mod first {
    pub fn say_hello() {
        println!("Hello from first module");
    }
}

mod second {
    pub fn say_hello() {
        println!("Hello from first module");
    }
}
// use 'use' keyword to import the function from the module
use first::say_hello;
use second::say_hello as second_say_hello;

#[test]
fn test_use_module() {
    say_hello();
    second_say_hello();
}

// Module di File lain
// filename will be module
// as default separated file will not be included
// These lines declare two modules, third and fourth
mod fourth;
mod third;

// use third::say_hello as third_say_hello;
// use fourth::say_hello as fourth_say_hello;
// or
// use third::*;
// use fourth::*;
// or
use fourth::say_hello as fourth_say_hello;
use third::{say_hello as third_say_hello, seconds::thirds::say_hello as third_say_hello2};
#[test]
fn separated_file_module() {
    third_say_hello();
    fourth_say_hello();
    third_say_hello2();
}

// Trait

// trait is same with interface in other language
// trait is a collection of methods that can be used in a struct
// trait is implemented using the impl keyword
trait CanSayHello {
    // default implementation
    // default implementation is automatically implemented
    fn hellow(&self) -> String {
        String::from("Hello, iam default implementation")
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!(
            "Hello, my name is {} and I say hello to {}",
            self.name, name
        )
    }
}

// trait as parameter
fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

#[test]
fn trait_feature() {
    let person = Person {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };

    say_hello_trait(&person);

    println!("{}", person.say_hello());
    println!("{}", person.say_hello_to("Noah"));
    println!("{}", person.hellow());
}

// multiple trait
trait CanSayGoodbye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodbye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!(
            "Goodbye, my name is {} and I say goodbye to {}",
            self.name, name
        )
    }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_multiple_trait() {
    let person = Person {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Noah"));

    hello_and_goodbye(&person);
}

// Trait as return type
struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye simple person, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!(
            "Goodbye simple person, my name is {} and I say goodbye to {}",
            self.name, name
        )
    }
}

impl CanSayHello for SimplePerson {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!(
            "Hello, my name is {} and I say hello to {}",
            self.name, name
        )
    }
}

fn create_person(name: String) -> impl CanSayGoodbye + CanSayHello {
    return SimplePerson { name };
}

#[test]
fn test_trait_as_return_type() {
    let person = create_person(String::from("Sam"));
    println!("{}", person.good_bye());
    println!("{}", person.say_hello());
}

// Solve conflict method name in Trait
struct Person2 {
    name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

// create method for Person2
impl Person2 {
    fn say_hello(&self, name: &str) {
        println!(
            "Hello Person2, my name is {} and I say hello to {}",
            self.name, name
        );
    }
}

// implement Trait for Person2
impl CanSayHello for Person2 {
    fn say_hello(&self) -> String {
        format!("Hello CanSayHello Person2, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!(
            "Hello CanSayHello Person2, my name is {} and I say hello to {}",
            self.name, name
        )
    }
}

#[test]
fn test_solve_conflict_method_name_in_trait() {
    let person = Person2 {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };

    // agar tidak salah memanggail method, kita perlu menggunakan Trait name atau Struct name

    // to access method to Struct, we need to use Struct name
    Person2::say_hello(&person, "Noah");

    // to access method to Trait, we need to use Trait name
    println!("{}", CanSayHello::say_hello(&person));
}

// SUPER TRAIT

trait SuperTrait: CanSayHello + CanSayGoodbye {
    // default implementation
    fn super_hellow(&self) {
        println!("Hello, iam default implementation SuperTrait");
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }

    fn super_say_hello(&self) -> String;
}

struct SuperPerson {
    name: String,
}

impl CanSayHello for SuperPerson {
    fn say_hello(&self) -> String {
        format!("Hello CanSayHello SuperPerson, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!(
            "Hello CanSayHello SuperPerson, my name is {} and I say hello to {}",
            self.name, name
        )
    }
}

impl CanSayGoodbye for SuperPerson {
    fn good_bye(&self) -> String {
        format!("Goodbye SuperPerson, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!(
            "Goodbye SuperPerson, my name is {} and I say goodbye to {}",
            self.name, name
        )
    }
}

// must implement all trait in SuperTrait
// which is CanSayHello and CanSayGoodbye
impl SuperTrait for SuperPerson {
    fn super_say_hello(&self) -> String {
        format!("Hello SuperTrait, my name is {}", self.name)
    }
}

#[test]
fn test_super_trait() {
    let super_person = SuperPerson {
        name: String::from("Rizky"),
    };
    println!("{}", super_person.say_hello());
    println!("{}", super_person.good_bye());
    println!("{}", super_person.super_say_hello());
    super_person.super_hellow()
}
// GENERIC
// In Rust, generic is a way to create a function or struct that can be used with different types
// Generic is implemented using the <> syntax
// Generic is used to create a function or struct that can be used with different types

// in Struct
struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 1, y: 2 };
    println!("Point integer: {}", integer.x);
    println!("Point integer: {}", integer.y);

    let float: Point<f64> = Point::<f64> { x: 1.1, y: 2.2 };
    println!("Point float: {}", float.x);
    println!("Point float: {}", float.y);
}

//  in Enum

// enum option builtin in rust
// enum Option<T> {
//     Some(T),
//     None,
// }

#[test]
fn test_generic_enum() {
    let some_number = Option::<i32>::Some(5);

    match some_number {
        Option::Some(number) => println!("Number is {}", number),
        Option::None => println!("No number"),
    }

    let no_number: Option<i32> = Option::None;

    match no_number {
        Option::Some(number) => println!("Number is {}", number),
        Option::None => println!("No number"),
    }
}

// generic type bound
// generic type bound is a way to specify the type of the parameter

struct Hi<T: CanSayGoodbye> {
    value: T,
}

#[test]
fn test_generic_type_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Rizky"),
        },
    };

    println!("{}", hi.value.good_bye());
}

// generic function
// PartialOrd is a trait that is used to compare two values
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn test_generic_function() {
    let result = min::<i32>(3, 2);
    println!("{}", result);

    let result = min(3, 2);
    println!("{}", result);
}

// generic method
struct NewPoint<T> {
    x: T,
    y: T,
}

impl<T> NewPoint<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = NewPoint::<i32> { x: 1, y: 2 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());

    // rust can infer the type of the parameter
    let point2 = NewPoint { x: 1.1, y: 2.2 };
    println!("{}", point2.get_x());
    println!("{}", point2.get_y());
}

// generic trait
trait GetValue<T> {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for NewPoint<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}

#[test]
fn test_generic_trait() {
    let point = NewPoint { x: 1, y: 2 };
    println!("{}", point.get_value());
}

// where clause
// where clause is a way to specify the type of the parameter
// where clause is used in trait
trait NewGetValue<T>
where
    T: PartialOrd,
{
    fn get_value_new(&self) -> &T;
}

impl<T> NewGetValue<T> for NewPoint<T>
where
    T: PartialOrd,
{
    fn get_value_new(&self) -> &T {
        &self.x
    }
}

// or using where clause
struct HiWithWhere<T>
where
    T: CanSayGoodbye + CanSayHello,
{
    value: T,
}

// Default Generic Type
// Default Generic Type is a way to specify the type of the parameter
// Default Generic Type is used in function
// use = to specify the default type
struct DefaultGenericType<T = i32> {
    value: T,
}

#[test]
fn test_default_generic_type() {
    let default_generic_type = DefaultGenericType::<i32> { value: 1 };
    println!("{}", default_generic_type.value);

    let default_generic_type = DefaultGenericType { value: 1 };
    println!("{}", default_generic_type.value);
}

// overloadble operator
// overloadble operator is a way to create a function with the same name but different parameters
// overloadble operator is used in function
// use different parameters to distinguish the function

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::{cmp::Ordering, fmt::Debug, ops::Add};
use std::{result, string, vec};

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_overloadble_operator() {
    let apple1 = Apple { quantity: 1 };
    let apple2 = Apple { quantity: 2 };
    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

// Optional Values
// rust not have null or undefined
// penggantinya rush memiliki Enum Option
// None     -> nilai kosong
// Some(T)  -> nilai tidak kosong tapi bebas tipe datanya

fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x * 2),
    }
}

#[test]
fn test_enum_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result_none = double(None);
    println!("{:?}", result_none);
}

// Comparing
// Module/Crate core::cmp

// PartialEq merupakan trait yang digunakan untuk melakukan perbandingan antara dua nilai
// mengimplementasikan PartialEq ke struct Apple
impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

// mengimplementasikan PartialOrd ke struct Apple
impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

// atau bisa menggunakan macro #[derive(Debug, PartialEq, PartialOrd)] pada struct Apple

#[test]
fn test_comparing() {
    let apple1 = Apple { quantity: 1 };
    let apple2 = Apple { quantity: 2 };
    println!("apple1 == apple2 {}", apple1 == apple2);
    println!("apple1 != apple2 {}", apple1 != apple2);
    println!("apple1 >= apple2 {}", apple1 >= apple2);
    println!("apple1 <= apple2 {}", apple1 <= apple2);
}

// String Manipulation
#[test]
fn test_string_manipulation() {
    let s = String::from("Rizky Sam Pratama");
    // let s_slice = "Sam";

    println!("String: {}", s);
    println!("String: {}", s.to_lowercase());
    println!("String: {}", s.to_uppercase());
    println!("String: {}", s.replace("Sam", "Pratama"));
    println!("String: {}", s.replace("Pratama", "Ganteng"));
    println!("String: {}", s.contains("Sam"));
    println!("String: {}", s.contains("Rizky"));
    println!("String: {}", s.len());
}

// Formating String
/**
 * Key Point:
 * Use {} for simple, user-friendly output
 * Use {:?} for debugging and development
 * Use {:#?} when you need to inspect complex nested structures
 * Custom types need to implement Display for {} to work
 * Most types automatically implement Debug when you add #[derive(Debug)]
 */
#[test]
fn test_formating_string() {
    let person = Person {
        name: String::from("Rizky"),
        middle_name: String::from("Sam"),
        last_name: String::from("Pratama"),
        age: 29,
    };

    // println!("Person name: {}", person); // `Person` doesn't implement `std::fmt::Display` , println without format
    // println!("Person middle name: {:}", person); // `Person` doesn't implement `std::fmt::Display` , println with format
    // println!("Person name: {:?}", person); // `Person` implements `std::fmt::Debug`, debug not pretty format

    // Display {}
    // tipe data primitive (string slice, number dll) -> Display
    println!("Person name: {}", person.name); // `Person` implements `std::fmt::Debug`, debug pretty format
                                              // Output: Person name: Rizky

    println!("Person name: {:}", person.name); // `Person` implements `std::fmt::Debug`, debug pretty format
                                               // Output: Person name: Rizky

    println!("--------------------------------");

    // Debug {:?}
    // tipe data kompleks (struct, enum, array, tuple dll) -> Debug
    println!("Person: {:?}", person);
    // Output: Person { name: "Rizky", middle_name: "Sam", last_name: "Pratama", age: 29 }

    println!("Person: {:#?}", person);
    // Output
    // Person: Person {
    //     name: "Rizky",
    //     middle_name: "Sam",
    //     last_name: "Pratama",
    //     age: 29,
    // }

    let cat = Category {
        name: String::from("Cat"),
        description: String::from("Cat description"),
    };
    println!("Category: {:#?}", cat);
    // Output
    // Category: Category {
    //     name: "Cat",
    //     description: "Cat description",
    // }
}

struct Category {
    name: String,
    description: String,
}

use std::fmt::{Display, Formatter};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

// Closure
// Fungsi tanpa nama
#[test]
fn test_closure() {
    // |value1: i32, value2: i32| -> i32 { value1 + value2 } [closure]
    // let sum: fn(i32, i32) -> i32 [nama variable]
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    let result = sum(1, 2);
    println!("{}", result);
}

// Closure as parameter
fn print_with_filter(value: String, f: fn(String) -> String) {
    let result = f(value);
    println!("{}", result);
}

#[test]
fn print_closure_as_params() {
    let s = String::from("Rizky Sam Pratama");

    let filter = |value: String| -> String { value.to_uppercase() };
    print_with_filter(s, filter);
}

// Closure dari Function
fn my_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_from_function() {
    let s = String::from("Rizky Sam Pratama");
    print_with_filter(s, my_uppercase);
}

// Closure Scope
// kita dapat menangkap data di scope yang sama
// namun closure scope ini dapat membingungkan jika dipakai terlalu banyak
#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Incremented");
    };

    increment();
    increment();

    println!("Counter: {}", counter);
}

// alternatif closure scope
// yaitu menggunakan Closure dengan struct

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
        println!("Incremented");
    }
}

#[test]
fn test_alternatif_closure_scope() {
    let mut counter = Counter { count: 0 };
    let mut increment = || counter.increment();
    increment();
    increment();

    println!("Counter: {:?}", counter);
}

// Collection Type
// Collection Type adalah tipe data yang dapat menampung beberapa data sekaligus
// Collection Type di Rust terdiri dari 3 macam yaitu:

// 1. Sequence: tipe data collection yang memiliki index
// 1.1 Vec (vector) : urutannya sesuai dengan yang kita inginkan
// 1.1.1 Menambahkan data ke vector dilakukan dibagian belakang
// 1.1.2 Cocok implementasi stack (tumpukan) dan Last in, First out
// 1.1.3 Disimpan di Heap
// 2. VecDeque (vector double ended queue)
// 2.1 Sama seperti Vec, namun memiliki kemampuan untuk menambahkan data di awal dan akhir
// 2.2 Cocok implementasi queue (antrian) dan First in, First out
// 2.3 Disimpan di Heap
// 3. LinkedList (implementasi sequence menggunakan tipe data linked list)
// 3.1 Linkedlist efisien untuk menambahkan dan mengurangkan data, sangat cocok ketika kita butuh Sequence yang tidak terprediksi ukurannya
// 3.2 Disimpan di Heap
// 3.3 performa linkedlist tidak sebaik vector

// 2. Set
// 3. Map

#[test]
fn test_vector_collection() {
    let mut names: Vec<String> = Vec::<String>::new();

    names.push(String::from("Rizky"));
    names.push(String::from("Sam"));
    names.push(String::from("Pratama"));
    names.pop();

    println!("Names: {:?}", names);
    println!("Names length: {}", names.len());
    println!("Names first: {}", names[0]);
    println!("Names last: {}", names[names.len() - 1]);

    // harus dikirim reference/pointer agar tidak dihapus ownership nya dari variable names
    for name in &names {
        println!("{}", name);
    }

    println!("Names: {:?}", names);
}

#[test]
fn test_vector_dequeue_collection() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();

    names.push_front(String::from("Rizky"));
    names.push_front(String::from("Sam"));
    names.push_back(String::from("Pratama"));
    // names.pop_back();

    println!("Names: {:?}", names);
    println!("Names length: {}", names.len());
    println!("Names first: {}", names[0]);
    println!("Names last: {}", names[names.len() - 1]);

    // harus dikirim reference/pointer agar tidak dihapus ownership nya dari variable names
    for name in &names {
        println!("{}", name);
    }

    println!("Names: {:?}", names);
}

#[test]
fn test_linkedlist_collection() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();

    names.push_front(String::from("Rizky"));
    names.push_front(String::from("Sam"));
    names.push_back(String::from("Pratama"));
    // names.pop_back();

    println!("Names: {:?}", names);
    println!("Names length: {}", names.len());

    // harus dikirim reference/pointer agar tidak dihapus ownership nya dari variable names
    for name in &names {
        println!("{}", name);
    }

    println!("Names: {:?}", names);
}

// MAP Type
// Key - Value
// Key harus unik
// Key bisa berupa tipe data apapun
// Value bisa berupa tipe data apapun
// 1. HashMap: key tidak diurutkan (tapi lebih cepat get dan set data)
// 2. BTreeMap: key diurutkan

#[test]
fn test_hashmap() {
    let mut my_map: HashMap<String, String> = HashMap::new();

    // add to hashmap
    my_map.insert(String::from("name"), String::from("Sam"));
    my_map.insert(String::from("age"), String::from("20"));

    println!("My map: {:#?}", my_map);

    let name = my_map.get("name"); // masih berupa options
                                   // unwrap dipakai untuk mengambil value dari options

    println!("Name: {}", name.unwrap());

    for (key, value) in &my_map {
        println!("{}: {}", key, value);
    }

    let name_2: Option<String> = None;

    // unwrap kurang aman karena jika value nya None maka akan panic
    // println!("Name: {}", name_2.unwrap());
    println!(
        "Name None: {}",
        name_2.unwrap_or("No name provided".to_string())
    );
}

#[test]
fn test_btreemap() {
    let mut my_btreemap: BTreeMap<String, String> = BTreeMap::new(); // key diurutkan
    my_btreemap.insert("Name".to_string(), "Rizky".to_string());
    my_btreemap.insert("Age".to_string(), "20".to_string());
    my_btreemap.insert("Address".to_string(), "Jakarta".to_string());

    // BTree ini isinya tuple jika di extract
    // output:
    // ("Address", "Jakarta")
    // ("Age", "20")
    // ("Name", "Rizky")
    // diurutkan key nya
    for entry in &my_btreemap {
        println!("{:?}", entry);
    }
}

// SET Type
// tipe data collection, dimana didalam data tersebut tidak ada duplikasi data
// jika sudah ada data tersebut, maka data tersebut akan diabaikan
// Set tidak bisa diakses dengan index
// 1. HashSet: key tidak diurutkan + unik data
// 2. BTreeSet: key diurutkan + unik data

#[test]
fn test_hashset() {
    let mut my_hashset: HashSet<String> = HashSet::new();
    my_hashset.insert(String::from("Rizky"));
    my_hashset.insert(String::from("Sam"));
    my_hashset.insert(String::from("Pratama"));
    my_hashset.insert(String::from("Pratama"));
    my_hashset.insert(String::from("Sam"));

    println!("My hashset: {:#?}", my_hashset);

    for name in &my_hashset {
        println!("{}", name);
    }
}

#[test]
fn test_btreeset() {
    let mut my_btreeset: BTreeSet<String> = BTreeSet::new();
    my_btreeset.insert(String::from("Rizky"));
    my_btreeset.insert(String::from("Sam"));
    my_btreeset.insert(String::from("Pratama"));
    my_btreeset.insert(String::from("Pratama"));
    my_btreeset.insert(String::from("Sam"));
    my_btreeset.insert(String::from("Ariel"));
    my_btreeset.insert(String::from("1"));

    println!("My btreeset: {:#?}", my_btreeset);

    for name in &my_btreeset {
        println!("{}", name);
    }
}

// Iterator
// Iterator adalah tipe data collection, dimana digunakan untuk mengakses data secara efisien

#[test]
fn test_iterator() {
    let my_array: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut iterator = my_array.iter();

    // iterator manual
    while let Some(value) = iterator.next() {
        println!("a {}", value);
    }

    println!("{:?}", iterator); // empty iterator, because all data has been consumed

    // iterator for in but we need to create new iterator
    // secara tidak langsung .iter() akan di panggil dalam for loop
    for value in my_array {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Vector: {:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("Sum: {}", sum);

    let count: usize = vector.iter().count();
    println!("Count: {}", count);

    /*
     * why we use Vec<i32> for variable doubled?
     * The map operation is transforming the values
     * We're creating new values (multiplying by 2)
     * So we can specify that we want the result to be a vector of new i32 values
     * map creates new values, so we can choose what type we want for the output
     */
    let doubled: Vec<i32> = vector.iter().map(|value: &i32| value * 2).collect();
    println!("Doubled: {:?}", doubled);

    println!("Vector: {:?}", vector);

    /*
     * why we use Vec<i32> for variable odd?
     * filter just selects existing elements
     * It doesn't transform them
     * It keeps the original references
     * filter just passes through existing references, so we have to keep the reference type
     */
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("Odd: {:?}", odd);

    println!("Vector: {:?}", vector);
}
