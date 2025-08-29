// There are three types of structures ("structs") that can be created using the struct keyword:
// Tuple structs, which are, basically, named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A un

fn main() {
    println!("Hello, world!");
}
