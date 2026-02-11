use std::fs::File;
use std::io::{self, Read};

// Exercise 1: Option Handling

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn option_example() {
    let user = find_user(1);

    match user {
        Some(name) => println!("User found: {}", name),
        None => println!("User not found"),
    }
}

// Exercise 2: Result Handling

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn result_example() {
    match read_file("data.txt") {
        Ok(data) => println!("File data:\n{}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

// Exercise 3: Custom Error Type

#[derive(Debug)]
enum AppError {
    InvalidInput,
    IoError(io::Error),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

fn validate_input(n: i32) -> Result<i32, AppError> {
    if n < 0 {
        Err(AppError::InvalidInput)
    } else {
        Ok(n)
    }
}

// Exercise 4: Error Propagation

fn process_file(path: &str) -> Result<(), AppError> {
    let data = read_file(path)?; // propagate error

    println!("Processing: {}", data);

    Ok(())
}

// Exercise 5: panic! Example

fn panic_example() {
    let x = 0;

    if x == 0 {
        panic!("Critical error: x cannot be zero");
    }
}

// Main

fn main() {
    println!("--- Day 3: Error Handling ---");

    // Option
    option_example();

    // Result
    result_example();

    // Custom error
    match validate_input(-5) {
        Ok(v) => println!("Valid input: {}", v),
        Err(e) => println!("Validation error: {:?}", e),
    }

    // Propagation
    if let Err(e) = process_file("data.txt") {
        println!("Process failed: {:?}", e);
    }

    // Panic (comment after testing once)
    // panic_example();

    println!("--- End of Day 3 ---");
}
