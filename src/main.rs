use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

fn get_random_num_in_range() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn main() {
    println!("Guess the number");
    let mut tries = 0;
    let mut random_number = get_random_num_in_range();

    loop {
        tries += 1;

        println!("What is your guess (between 1 and 100) ? ");

        let mut user_number = String::new();

        io::stdin()
            .read_line(&mut user_number)
            .expect("Error reading user input.");

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
                println!("You win 🎉🎊 ! Mystery number found in {} trie(s).", tries);

                loop {
                    let mut restart = String::new();

                    println!("Would you like to restart the game ? (\"yes\" / \"no\") ");

                    io::stdin()
                        .read_line(&mut restart)
                        .expect("Error reading user input.");

                    let restart = restart.trim();

                    match restart {
                        "yes" => {
                            random_number = get_random_num_in_range();
                            tries = 0;

                            break;
                        }
                        "no" => {
                            println!("Thanks for playing Guess the number game !");
                            println!("See you soon. :)");

                            exit(0)
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}
