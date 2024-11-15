// Rust Basics Cheat Sheet

// -----------------------------
// 1. VARIABLES AND TYPES
// -----------------------------

// Variables
let immutable = 42;
let mut mutable = 42;
const CONSTANT: i32 = 42;
static STATIC: i32 = 42;

// Basic Types
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'a';
let string: String = String::from("Hello");
let str_slice: &str = "World";

// Compound Types
let tuple: (i32, f64, char) = (42, 3.14, 'a');
let array: [i32; 5] = [1, 2, 3, 4, 5];
let vector: Vec<i32> = vec![1, 2, 3, 4, 5];

// -----------------------------
// 2. CONTROL FLOW
// -----------------------------

// If Expression
let number = 7;
if number < 5 {
    println!("less than five");
} else if number > 5 {
    println!("greater than five");
} else {
    println!("equal to five");
}

// Match Expression
let value = Some(3);
match value {
    Some(3) => println!("three"),
    Some(x) => println!("got {}", x),
    None => println!("none"),
}

// Loops
// Loop with break
let mut counter = 0;
loop {
    counter += 1;
    if counter == 10 { break; }
}

// While loop
while counter > 0 {
    counter -= 1;
}

// For loop
for i in 0..5 {
    println!("{}", i);
}

// -----------------------------
// 3. FUNCTIONS
// -----------------------------

// Basic Function
fn add(a: i32, b: i32) -> i32 {
    a + b  // Implicit return
}

// Function with multiple parameters
fn greet(name: &str, greeting: &str) -> String {
    format!("{} {}", greeting, name)
}

// Function with references
fn modify(value: &mut i32) {
    *value += 1;
}

// -----------------------------
// 4. ERROR HANDLING
// -----------------------------

// Result Type
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using Result
let result = divide(10.0, 2.0);
match result {
    Ok(value) => println!("Result: {}", value),
    Err(e) => println!("Error: {}", e),
}

// Option Type
let some_value: Option<i32> = Some(42);
let none_value: Option<i32> = None;

// Using ?
fn try_parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    let x: i32 = s.parse()?;
    Ok(x)
}

// -----------------------------
// 5. OWNERSHIP BASICS
// -----------------------------

// Ownership Rules
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2
// println!("{}", s1);  // Error: s1 moved

// References
let s1 = String::from("hello");
let len = calculate_length(&s1);  // Borrowing

// Mutable References
let mut s = String::from("hello");
change(&mut s);  // Mutable borrowing

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// -----------------------------
// 6. COMMON COLLECTIONS
// -----------------------------

// Vec<T>
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);

// Accessing Vector
let third: &i32 = &vec[2];
match vec.get(2) {
    Some(third) => println!("Third element is {}", third),
    None => println!("No third element"),
}

// String Methods
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');
let len = s.len();
let slice = &s[0..5];

// HashMap
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Accessing HashMap
match scores.get("Blue") {
    Some(score) => println!("Blue team score: {}", score),
    None => println!("No score for Blue team"),
}

// -----------------------------
// 7. MODULES AND SCOPE
// -----------------------------

// Module Example
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn private_function() {
        // Not accessible outside module
    }
}

// Using Modules
use math::add;

// Path
use std::collections::HashMap;
use std::io::{self, Write};

// -----------------------------
// 8. BASIC TRAITS
// -----------------------------

// Implementing Traits
struct Rectangle {
    width: u32,
    height: u32,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

// Common Traits
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// -----------------------------
// 9. STANDARD INPUT/OUTPUT
// -----------------------------

// Printing
println!("Normal text");
print!("No newline");
eprintln!("Error message");

// Formatting
println!("Formatted: {}", 42);
println!("Multiple: {}, {}", 1, 2);
println!("Named: {value}", value = 42);

// Reading Input
use std::io;
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");

// -----------------------------
// 10. STRING MANIPULATION
// -----------------------------

// String Methods
let s = String::from("hello world");
let upper = s.to_uppercase();
let lower = s.to_lowercase();
let trimmed = s.trim();
let contains = s.contains("world");
let replaced = s.replace("world", "Rust");

// String Conversion
let s = "42".to_string();
let n = "42".parse::<i32>().unwrap();

// String Concatenation
let s1 = String::from("Hello");
let s2 = String::from(" World");
let s3 = s1 + &s2;  // Note: s1 is moved