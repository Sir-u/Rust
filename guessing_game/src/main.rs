extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The Secret number is: {}", secret_number);
    
    println!("Please input your quess.");

    let mut quess = String::new();

    io::stdin().read_line(&mut quess).expect("Failed to read line");

    println!("You quessed: {}", quess);
}