// Globals are declared outside all other scopes.

// A possibly mutable variable with 'static lifetime. 
// The static lifetime is inferred and does not have to be specified. 
// Accessing or modifying a mutable static variable is unsafe.
// &str is a string slice, an immutable reference to a string.
static LANGUAGE: &str = "Rust"; 
static mut CURRENT: Language = Language::Rust;
// const is always immutable
const THRESHOLD: i32 = 10; 

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

#[derive(Debug)]
enum Language {
    Rust,
    Cpp,
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line

    unsafe {
        CURRENT = Language::Cpp;
        match CURRENT {
            Language::Rust => println!("Language is Rust"),
            Language::Cpp => println!("Language is C++"),
        }
    }
}