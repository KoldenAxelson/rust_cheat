// Rust Intermediate Concepts Cheat Sheet

// -----------------------------
// 1. ADVANCED PATTERN MATCHING
// -----------------------------

// Pattern Types
let x = 1;
match x {
    1 | 2 => println!("one or two"),
    3..=5 => println!("three through five"),
    _ => println!("other"),
}

// Destructuring
let point = (0, 0);
let (x, y) = point;

struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };
let Point { x, y: y_value } = p;

// Match Guards
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("less than five"),
    Some(x) => println!("{}", x),
    None => (),
}

// Binding with @
let msg = Some(4);
match msg {
    Some(x @ 1..=5) => println!("got a range element {}", x),
    Some(n) => println!("other number: {}", n),
    None => (),
}

// -----------------------------
// 2. ADVANCED TRAITS
// -----------------------------

// Default Implementation
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    
    fn summarize_author(&self) -> String;
}

// Trait Bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple Trait Bounds
fn process<T: Summary + Clone>(item: &T) -> T {
    item.clone()
}

// Where Clauses
fn complex_function<T, U>(t: &T, u: &U) -> i32
where
    T: Clone + Summary,
    U: Clone + Default,
{
    0
}

// Associated Types
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
}

// -----------------------------
// 3. ADVANCED GENERICS
// -----------------------------

// Generic Structs
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// Const Generics
struct Array<T, const N: usize> {
    data: [T; N],
}

// Multiple Type Parameters
struct KeyValue<K, V> {
    key: K,
    value: V,
}

// Default Type Parameters
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// -----------------------------
// 4. LIFETIMES
// -----------------------------

// Lifetime Syntax
struct NamedReference<'a> {
    name: &'a str,
}

// Multiple Lifetimes
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Static Lifetime
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

// Lifetime Elision
fn first_word(s: &str) -> &str {  // Lifetimes elided
    &s[..1]
}

// -----------------------------
// 5. SMART POINTERS
// -----------------------------

// Box<T>
let b = Box::new(5);
let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

// Rc<T>
use std::rc::Rc;
let value = Rc::new(String::from("shared data"));
let value2 = Rc::clone(&value);

// RefCell<T>
use std::cell::RefCell;
let data = RefCell::new(5);
*data.borrow_mut() += 1;

// Combining Smart Pointers
let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));

// -----------------------------
// 6. CONCURRENCY BASICS
// -----------------------------

// Threads
use std::thread;
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();

// Channels
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(String::from("hi")).unwrap();
});
println!("Got: {}", rx.recv().unwrap());

// Mutex
use std::sync::Mutex;
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}

// Arc for Thread-Safe Sharing
use std::sync::Arc;
let counter = Arc::new(Mutex::new(0));

// -----------------------------
// 7. ITERATORS
// -----------------------------

// Custom Iterator
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

// Iterator Adaptors
let v = vec![1, 2, 3];
let sum: i32 = v.iter()
    .map(|x| x * 2)
    .filter(|x| x > &3)
    .sum();

// Consuming Adaptors
let total: i32 = v.iter().sum();
let has_three = v.iter().any(|&x| x == 3);

// Chain and Zip
let a = [1, 2, 3];
let b = [4, 5, 6];
let chained: Vec<i32> = a.iter().chain(b.iter()).cloned().collect();
let zipped: Vec<(i32, i32)> = a.iter().zip(b.iter()).map(|(&x, &y)| (x, y)).collect();

// -----------------------------
// 8. ADVANCED ERROR HANDLING
// -----------------------------

// Custom Error Types
#[derive(Debug)]
enum CustomError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::IoError(e) => write!(f, "IO error: {}", e),
            CustomError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

// Error Conversion
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::IoError(err)
    }
}

// Using Custom Errors
fn process_data() -> Result<(), CustomError> {
    let _file = std::fs::File::open("data.txt")?;
    Ok(())
}

// -----------------------------
// 9. ADVANCED CLOSURES
// -----------------------------

// Move Closures
let string = String::from("hello");
let closure = move || println!("{}", string);

// FnOnce, FnMut, and Fn
fn call_once<F>(f: F) where F: FnOnce() {
    f();
}

fn call_mut<F>(mut f: F) where F: FnMut() {
    f();
}

fn call_fn<F>(f: F) where F: Fn() {
    f();
}

// Closure Type Inference
let add_one = |x: i32| x + 1;
let add_two = |x| x + 2;  // Type inferred

// -----------------------------
// 10. MACROS
// -----------------------------

// Macro Rules
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Macro Repetition
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Using Macros
say_hello!();
say_hello!("Rust");
let vec = vector![1, 2, 3];