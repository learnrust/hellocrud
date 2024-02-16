pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}

pub fn hello_world() -> String {
    String::from("Hello, World!")
}

#[rustfmt::skip]
pub fn hello() {
    println!("Hello, World!");
}
