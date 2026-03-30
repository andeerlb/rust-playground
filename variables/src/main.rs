
// This is a simple Rust program that demonstrates the use of variables, constants, and shadowing.

// constants are immutable by default and must be declared with a type
const TAX_RATE: f64 = 0.07; // constant variable for tax rate

fn main() {
    // variables are immutable by default, but we can make them mutable using the `mut` keyword
    let apples_count = 50;
    let mut apples = 50;
    println!("We have {} apples.", apples);
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    println!("We have {} fruits in total.", fruits);
    println!("We have {} oranges.", oranges);
    println!("apples count is {}", apples_count);

    apples = 40;
    println!("Now we have {} apples.", apples);

    // we can also change the type of a variable by reassigning it to a value of a different type
    // it is called "shadowing" in Rust
    // we can shadow a variable by declaring a new variable with the same name
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    println!("We have {} grams of protein.", grams_of_protein);

    // reading constant
    let income = 50000.0;
    println!("The tax rate is {}.", TAX_RATE);
    println!("The income is {}.", income);
}

