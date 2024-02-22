use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let mut tries = 0;
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        tries += 1;

        println!("What is your guess (between 1 and 100) ? ");

        let mut user_number = String::new();

        io::stdin()
            .read_line(&mut user_number)
            .expect("Error reading using input.");

        let user_number: u32 = match user_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number (in given range)!");
                continue;
            }
        };

        match user_number.cmp(&random_number) {
            Ordering::Greater => println!("Too big !"),
            Ordering::Less => println!("Too small !"),
            Ordering::Equal => {
                println!("You win 🎉🎊 ! Mystery number found in {} tries.", tries);
                break;
            }
        }
    }
}
