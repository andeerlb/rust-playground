// There are three types of structures ("structs") that can be created using the struct keyword:
// Tuple structs, which are, basically, named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)] // This will automatically implement the Debug trait for Person
struct Person {
    name: String,
    age: u8, // u8 means "unsigned 8-bit integer"
}

// A unit struct
struct Unit;

// a structure with two fields
struct Point {
    x: f32, // f32 means "32-bit floating point"
    y: f32,
}

// A structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

struct Pair(i32, f32); // tuple struct

fn main() {
    // create struct with field init shorthand
    let name = String::from("Anderson"); // String::from creates a new String
    let age = 29;
    let person = Person { name, age };
    println!("{:?}", person); // {:?} will print the Person struct, ? means "use the Debug formatter"


    // instantiate a Point
    let p1 = Point { x: 0.9, y: 0.2 };
    let p2 = Point { x: 15.2, y: 3.4 };
    println!("printing ({}, {})", p1.x, p1.y);

    // make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..p2 };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = p1;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
