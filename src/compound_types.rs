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