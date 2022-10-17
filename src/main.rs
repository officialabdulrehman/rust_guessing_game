use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");
    let guess: i32 = guess.trim().parse().expect("guess is missing");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small :/"),
        Ordering::Greater => println!("Too big :("),
        Ordering::Equal => println!("You won the lottery!~ :)"),
    }
}
