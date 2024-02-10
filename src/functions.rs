pub fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}

pub fn multiply(a: i32, b: i32) -> i32 {
    let product = a * b;
    product
}

pub fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}

pub fn hello_world() -> String {
    let greeting = String::from("Hello, World!");
    greeting
}

pub fn hello() {
    println!("Hello, World!");
}