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
