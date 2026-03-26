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
}

