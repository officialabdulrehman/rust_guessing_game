use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    let mut secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    print!("\x1B[2J\x1B[1;1H");
    println!("I dare you to Guess the unlucky number!");
    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Enter a number you dummy!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Too small :/")
            }
            Ordering::Greater => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Too big :(")
            }
            Ordering::Equal => {
                // println!("You won the lottery!~ :)");
                // break;
                print!("\x1B[2J\x1B[1;1H");
                println!("The number ran away you dummy!");
                println!("You think you can get a hold of the unlucky number ?");
                println!("Think again!");
                secret_number = rand::thread_rng().gen_range(1..=100);
            }
        }
        println!("Guess again kiddo!");
    }
}
