use std::io;

fn main() {
    println!("Guess the number game!");

    println!("Please enter guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line ");

    println!("This is your number: {guess}");
}
