use std::io;

fn main() {
    println!("Guess the number plz.");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin().readline(&mut guess)
}
