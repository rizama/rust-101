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
