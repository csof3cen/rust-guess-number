use std::io;

fn main() {
    println!("Guess the number");
    println!("What is your number (between 1 and 100) ? ");

    let mut user_number = String::new();

    io::stdin()
        .read_line(&mut user_number)
        .expect("Error reading using input.");

    println!("Your entered {}", user_number);
}
