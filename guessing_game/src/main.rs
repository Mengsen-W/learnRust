use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! is a define
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    println!("The secret number = {}", secret_number);

    println!("Please input your guess.");

    loop {
        // mut mean mutable
        // rust immutable be Default

        // String()::new is a static funciton of String
        let mut guess = String::new();

        // std::io::stdin()
        io::stdin()
            //& mean reference "nut guess" mean mutable
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // : u32 mean tell parse() to convert type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number type");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too small!"),
        }
    }
}
