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
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [12; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    let arr = [1, 2, 3, 4, 5];
    analyze_slice(&arr[1..4]);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}
