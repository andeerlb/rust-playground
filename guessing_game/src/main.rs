macro_rules! ask_for_input {
    ($msg:expr) => {
        {
            use std::io; // input/output library
            println!(">> {}", $msg);
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess
        }
    };
}

fn main() {
    println!("Welcome to the Guessing Game!"); // the !, means that it is a macro, not a function
    let guess = ask_for_input!("Please enter your guess:"); // using the custom macro

    println!("Hello, world!");
    println!("You guessed: {guess}");
}
