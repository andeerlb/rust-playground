
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
    #[allow(unused_variables)]
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    println!("We have {} grams of protein.", grams_of_protein);

    // reading constant
    let income = 50000.0;
    println!("The tax rate is {}.", TAX_RATE);
    println!("The income is {}.", income);

    // type aliasing
    type Kilometers = i32;
    let distance: Kilometers = 100;
    println!("The distance is {} kilometers.", distance);

    // compiler directives
    // we can use the `#[allow(dead_code)]` attribute to suppress warnings about unused code
    #[allow(unused_variables)]
    let unused_variable = 42;

    let mut seasons: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];
    println!("The seasons are: {:?}", seasons);

    let first: &str = seasons[0];
    println!("The first season is {}.", first);

    let second: &str = seasons[1];
    println!("The second season is {}.", second);

    seasons[0] = "Rainy";
    seasons[1] = "Dry";

    println!("The seasons are: {:?}", seasons);
    println!("The first season is {}.", first);
    println!("The second season is {}.", second);

    println!("\n--- Example: snapshot vs current read in array ---");
    let mut labels = [String::from("A"), String::from("B")];

    // Snapshot of the value currently inside position 0.
    let snapshot = labels[0].clone();
    labels[0] = String::from("Z");

    println!("snapshot (captured before change): {}", snapshot);
    println!("current labels[0] (read after change): {}", labels[0]);

    // True reference to the element only lives while we do not mutate the array.
    {
        let live_ref = &labels[1];
        println!("live_ref before mutation: {}", live_ref);
    }

    labels[1] = String::from("Y");
    println!("labels[1] after mutation: {}", labels[1]);
}

