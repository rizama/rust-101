// crate keyword is used to refer to the current crate
// crate is the root module of the crate
// crate is used for called other module in other module (not main.rs) (nested module)
// to use crate keyword, we need to call the module in the main.rs
// and then we can use crate keyword to call the module in other module
use crate::third::say_hello as third_say_hello;

pub fn say_hello() {
    println!("Hello from fourth module");

    third_say_hello();
}
