// TODO Arrays and Slices
// An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [],
// and their length, which is known at compile time, is part of their type signature [T: length].
// Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object;
// the first word is a pointer to the data, the second word is the length of the slice. The word size is the same as usize.
// usize = pointer + length, determined by the processor architecture, e.g. 64-bit systems have 64-bit pointers and lengths.
// Slices are often used to borrow a portion of an array without taking ownership of it. &[T]

// Arrays are fixed-size lists of elements of the same type
// Slices are dynamically-sized views into contiguous sequences of elements

use std::mem; // is a module in Rust's standard library that provides functions and types for working with memory-related operations.

// this fn borrows a slice of an array
fn analyze_slice(arr: &[i32]) {
    println!("first element of the slice: {}", arr[0]);
    println!("second element of the slice: {}", arr[1]);
    println!("The slice has {} elements", arr.len());
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    analyze_slice(&arr[1..4]);
}
