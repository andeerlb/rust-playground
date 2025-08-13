use std::cmp::Ordering;

macro_rules! ask_for_input {
    ($msg:expr) => {{
        use std::io;
        println!(">> {}", $msg);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess
    }};
}

fn main() {
    println!("Welcome to the Guessing Game!");
    let secret_number = rand::random_range(1..=10);

    loop {
        let guess: i32 = match ask_for_input!("Please enter your guess:")
            .trim() // remove \n
            .parse() // convert to i32
            {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 10.");
                continue;       
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
