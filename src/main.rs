use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use colored::*;

fn main() {
    loop{
        println!("Guess the number!");

        let mut rng = thread_rng();
        let secret_number: u32 = rng.gen_range(0..101);

        println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", &guess);

        // redeclared va shadowing
        let guess: u32 = match guess.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => continue
        };

        match  guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        };
    }
}
