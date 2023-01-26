use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your quess.");

    let mut quess = String::new();

    io::stdin().read_line(&mut quess).expect("Failed to read line");

    println!("You quessed: {}", quess);
}
