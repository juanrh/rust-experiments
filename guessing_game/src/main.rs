/*
https://doc.rust-lang.org/stable/book/guessing-game.html
*/
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let max_num:u32 = 10;
    println!("Guess the number (between 1 and {})!", max_num);

    let secret_number = rand::thread_rng().gen_range(1, max_num+1);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
     
        // Rust has gentle shadowing!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a number, bye!");
                break;
            },
        };
            
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break; // exit the loop
            }
        }
    }
}