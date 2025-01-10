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