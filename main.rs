// Exercise B: Variables & Data Types
fn run_b() {
    let x = 5;
    println!("x = {}", x);

    let mut y = 10;
    println!("y before = {}", y);
    y += 5;
    println!("y after = {}", y);

    let pi: f64 = 3.14159;
    let is_learning: bool = true;
    let grade: char = 'A';
    println!("pi = {}, is_learning = {}, grade = {}", pi, is_learning, grade);

    let z = "42";
    let parsed_z: u32 = z.parse().expect("Not a number!");
    println!("Parsed z = {}", parsed_z);
}

// Exercise D: Functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

fn factorial(n: u64) -> u64 {
    if n <= 1 { 
        1 
    } else { 
        n * factorial(n - 1) 
    }
}

fn main() {
    println!("--- Exercise B Results ---");
    run_b();
    
    println!("\n--- Exercise D Results ---");
    println!("Add: {}", add(3, 7));
    println!("Greet: {}", greet("Rustacean"));
    println!("First word: {}", first_word("Rust is great"));
    println!("Factorial(10): {}", factorial(10));
}
