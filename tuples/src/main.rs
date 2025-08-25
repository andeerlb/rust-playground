
// tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer_param, boolean_param) = pair;
    // `let` can be used to destructure a tuple (to bind its elements to variables)
    (boolean_param, integer_param)
}

// the following struct is for the activity
#[derive(Debug)] // This will allow us to print the struct using `{:?}`
// `{:?}` is a special format specifier for debugging output
struct MatrixInteger(i32, i32, i32, i32); // i32 is a 32-bit signed integer

#[derive(Debug)]
struct MatrixFloat(f32, f32, f32, f32); // f32 is a 32-bit floating point number

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // values can be extracted from the tuple using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Destructuring the tuple into individual variables
    let (a, b, c, d, e, f, g, h, i, j, k, l) = long_tuple;
    println!("Destructured values: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i, j, k, l);

    // tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("Reversed pair is {:?}", reverse(pair));

    // to create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    let one_element_tuple = (42,);
    println!("one element tuple: {:?}", one_element_tuple);

    // tuples can be destrcutured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = MatrixInteger(1, 2, 3, 4);
    println!("{:?}", matrix);

    let matrix2 = MatrixFloat(1.0, 2.0, 3.0, 4.0);
    println!("{:?}", matrix2);
}
