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

fn say_hello() {
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
    say_hello();
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
