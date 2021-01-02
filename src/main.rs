use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Welcome to the Guessing game.");

    let secret_number = generate_secret_name();

    loop {

        let mut guess = String::new();

        println!("Enter input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You number guesses {}", guess);
                println!("You win!");
                break;
            }
        }
    }

    fn generate_secret_name() -> u32 {
        return rand::thread_rng().gen_range(1..10);
    }
}