pub fn say_hello() {
    println!("Hello from third modules");
}

pub mod seconds {
    pub mod thirds {
        pub fn say_hello() {
            crate::fourth::say_hello(); // to access Module on top (option 1 use crate keyword) direct from main.rs
            super::super::say_hello(); // to access Module on top (option 2 use super keyword) going up step by step
        }
    }
}
